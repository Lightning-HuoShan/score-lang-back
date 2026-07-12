//! 实时 MIDI 播放器
//!
//! 通过 midir 连接系统 MIDI 合成器，实时播放 Score

use std::thread;
use std::time::{Duration as StdDuration, Instant};

use midir::MidiOutput;

use analysis::kind::score::{Score, MeasureEvent, LocalControl};
use analysis::kind::note_value::Duration;

/// 播放速度倍率
const DEFAULT_PPQ: u16 = 480;

/// 带绝对时间戳的 MIDI 事件
struct TimedMidiEvent {
    /// 从播放开始的微秒数
    timestamp_us: u64,
    data: Vec<u8>,
}

/// 将 Score 转换为带时间戳的 MIDI 事件序列
fn build_event_timeline(score: &Score, ppq: u16) -> Vec<TimedMidiEvent> {
    let bpm = score.global_bpm().max(1) as u64;
    let us_per_tick = 60_000_000 / (bpm * ppq as u64);
    let mut events = Vec::new();
    let mut current_tick: u64 = 0;

    // 全局速度事件（放在最前面）
    let us_per_quarter = 60_000_000 / bpm;
    events.push(TimedMidiEvent {
        timestamp_us: 0,
        data: vec![0xFF, 0x51, 0x03,
            (us_per_quarter >> 16) as u8,
            (us_per_quarter >> 8) as u8,
            us_per_quarter as u8],
    });

    for (track_idx, track) in score.tracks.iter().enumerate() {
        let channel = (track_idx as u8).min(15);

        // 乐器设置（时间 0）
        if let Some(ref inst) = track.instrument {
            events.push(TimedMidiEvent {
                timestamp_us: 0,
                data: vec![0xC0 | channel, inst.midi_program()],
            });
            let (cc_vol, vol_val) = inst.midi_cc_volume();
            events.push(TimedMidiEvent {
                timestamp_us: 0,
                data: vec![0xB0 | channel, cc_vol, vol_val],
            });
            let (cc_pan, pan_val) = inst.midi_cc_pan();
            events.push(TimedMidiEvent {
                timestamp_us: 0,
                data: vec![0xB0 | channel, cc_pan, pan_val],
            });
        }

        let mut track_tick: u64 = 0;

        for section in &track.sections {
            let repeat = section.repeat_times.unwrap_or(1);
            for _ in 0..repeat {
                for measure in &section.measures {
                    for event in &measure.events {
                        match event {
                            MeasureEvent::Note(note) => {
                                let ticks = duration_to_ticks(&note.duration, ppq);
                                if note.is_rest() {
                                    track_tick += ticks as u64;
                                } else if let Some(midi_note) = note.midi_key() {
                                    let vel = note.midi_velocity();
                                    events.push(TimedMidiEvent {
                                        timestamp_us: track_tick * us_per_tick,
                                        data: vec![0x90 | channel, midi_note, vel],
                                    });
                                    track_tick += ticks as u64;
                                    events.push(TimedMidiEvent {
                                        timestamp_us: track_tick * us_per_tick,
                                        data: vec![0x80 | channel, midi_note, 0],
                                    });
                                }
                            }
                            MeasureEvent::Chord(chord) => {
                                let ticks = duration_to_ticks(&chord.duration, ppq);
                                let notes = chord.midi_notes();
                                let vel = chord.midi_velocity();
                                // Note On
                                for &midi_note in &notes {
                                    events.push(TimedMidiEvent {
                                        timestamp_us: track_tick * us_per_tick,
                                        data: vec![0x90 | channel, midi_note, vel],
                                    });
                                }
                                track_tick += ticks as u64;
                                // Note Off
                                for &midi_note in &notes {
                                    events.push(TimedMidiEvent {
                                        timestamp_us: track_tick * us_per_tick,
                                        data: vec![0x80 | channel, midi_note, 0],
                                    });
                                }
                            }
                            MeasureEvent::Control(control) => {
                                if let LocalControl::LocalTempo(_) = control {
                                    // 局部速度变化暂不支持实时调整
                                }
                            }
                        }
                    }
                }
            }
        }

        if track_tick > current_tick {
            current_tick = track_tick;
        }
    }

    // 按时间戳排序
    events.sort_by_key(|e| e.timestamp_us);
    events
}

fn duration_to_ticks(duration: &Duration, ppq: u16) -> u32 {
    let quarter_notes = duration.quarter_notes();
    (quarter_notes * ppq as f32).round() as u32
}

/// 列出可用的 MIDI 输出端口
pub fn list_output_ports() -> Result<Vec<String>, String> {
    let midi_out = MidiOutput::new("ScoreAnalysis").map_err(|e| e.to_string())?;
    let ports = midi_out.ports();
    let mut result = Vec::new();
    for port in ports {
        let name = midi_out.port_name(&port).map_err(|e| e.to_string())?;
        result.push(name);
    }
    Ok(result)
}

/// 播放 Score
///
/// 使用系统默认 MIDI 输出端口（或第一个可用端口）实时播放
pub fn play_score(score: &Score) -> Result<(), String> {
    play_score_with_port(score, None)
}

/// 使用指定端口播放 Score
///
/// port_index: None 表示使用第一个可用端口
pub fn play_score_with_port(score: &Score, port_index: Option<usize>) -> Result<(), String> {
    let midi_out = MidiOutput::new("ScoreAnalysis").map_err(|e| e.to_string())?;

    let ports = midi_out.ports();
    if ports.is_empty() {
        return Err("没有找到可用的 MIDI 输出端口".to_string());
    }

    let port_idx = port_index.unwrap_or(0);
    if port_idx >= ports.len() {
        return Err(format!("端口索引 {} 超出范围（共 {} 个端口）", port_idx, ports.len()));
    }

    let port_name = midi_out.port_name(&ports[port_idx]).map_err(|e| e.to_string())?;
    println!("MIDI 输出端口: {}", port_name);

    let mut conn = midi_out.connect(&ports[port_idx], "score-analysis-player")
        .map_err(|e| e.to_string())?;

    // 构建事件时间线
    let events = build_event_timeline(score, DEFAULT_PPQ);

    if events.is_empty() {
        println!("没有可播放的事件");
        return Ok(());
    }

    let total_us = events.last().unwrap().timestamp_us;
    let total_secs = total_us as f64 / 1_000_000.0;
    println!("总时长: {:.1} 秒, 事件数: {}", total_secs, events.len());

    // 实时播放
    let start = Instant::now();
    for event in &events {
        let target = StdDuration::from_micros(event.timestamp_us);
        let elapsed = start.elapsed();
        if elapsed < target {
            thread::sleep(target - elapsed);
        }
        let _ = conn.send(&event.data);
    }

    // 等待最后一个音符结束
    thread::sleep(StdDuration::from_millis(500));

    // 关闭所有音符（All Notes Off）
    for ch in 0u8..16 {
        let _ = conn.send(&[0xB0 | ch, 123, 0]);
    }

    Ok(())
}

/// 非阻塞播放：在新线程中播放 Score
pub fn play_score_async(score: &Score) -> thread::JoinHandle<Result<(), String>> {
    let score = score.clone();
    thread::spawn(move || play_score(&score))
}
