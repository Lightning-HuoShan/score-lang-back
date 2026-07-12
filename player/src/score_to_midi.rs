//! Score → MIDI 事件转换器

use analysis::kind::score::{Score, Track, MeasureEvent, LocalControl};
use analysis::kind::note_value::Duration;

use crate::midi_writer::{MidiFile, MidiTrack, note_on, note_off, program_change, control_change, tempo_meta};

/// 默认 PPQ (ticks per quarter note)
const DEFAULT_PPQ: u16 = 480;

/// 将 Duration 转换为 ticks
fn duration_to_ticks(duration: &Duration, ppq: u16) -> u32 {
    let quarter_notes = duration.quarter_notes();
    (quarter_notes * ppq as f32).round() as u32
}

/// 将 Score 转换为标准 MIDI 文件 (Format 1)
///
/// - 轨道 0: 速度/控制轨道（全局 tempo）
/// - 轨道 1..N: 每个 Score Track 对应一个 MIDI 轨道
pub fn score_to_midi(score: &Score) -> MidiFile {
    score_to_midi_with_ppq(score, DEFAULT_PPQ)
}

pub fn score_to_midi_with_ppq(score: &Score, ppq: u16) -> MidiFile {
    let mut midi = MidiFile::new(1, ppq);

    // 轨道 0: 速度控制轨道
    let mut tempo_track = MidiTrack::new();
    tempo_track.set_name("Tempo");

    // 全局速度
    let bpm = score.global_bpm();
    let us_per_quarter = 60_000_000 / bpm.max(1) as u32;
    tempo_track.add_event(0, tempo_meta(us_per_quarter));

    midi.add_track(tempo_track);

    // 每个声部对应一个 MIDI 轨道
    for (idx, track) in score.tracks.iter().enumerate() {
        let channel = (idx as u8).min(15); // 最多 16 个通道
        let midi_track = convert_track(track, channel, ppq);
        midi.add_track(midi_track);
    }

    midi
}

/// 将单个 Track 转换为 MIDI 轨道
fn convert_track(track: &Track, channel: u8, ppq: u16) -> MidiTrack {
    let mut midi_track = MidiTrack::new();
    midi_track.set_name(&track.name);

    // 乐器设置
    if let Some(ref inst) = track.instrument {
        midi_track.add_event(0, program_change(channel, inst.midi_program()));
        let (cc_vol, vol_val) = inst.midi_cc_volume();
        midi_track.add_event(0, control_change(channel, cc_vol, vol_val));
        let (cc_pan, pan_val) = inst.midi_cc_pan();
        midi_track.add_event(0, control_change(channel, cc_pan, pan_val));
    }

    // 遍历段落
    for section in &track.sections {
        let repeat = section.repeat_times.unwrap_or(1);
        for _ in 0..repeat {
            for measure in &section.measures {
                for event in &measure.events {
                    match event {
                        MeasureEvent::Note(note) => {
                            let ticks = duration_to_ticks(&note.duration, ppq);
                            if note.is_rest() {
                                // 休止符：仅推进时间
                                midi_track.advance(ticks);
                            } else {
                                // 音符：note on → note off
                                if let Some(midi_note) = note.midi_key() {
                                    let vel = note.midi_velocity();
                                    midi_track.add_event(0, note_on(channel, midi_note, vel));
                                    midi_track.add_event(ticks, note_off(channel, midi_note, 0));
                                }
                            }
                        }
                        MeasureEvent::Chord(chord) => {
                            let ticks = duration_to_ticks(&chord.duration, ppq);
                            let notes = chord.midi_notes();
                            let vel = chord.midi_velocity();
                            // 所有音符同时发音
                            for &midi_note in &notes {
                                midi_track.add_event(0, note_on(channel, midi_note, vel));
                            }
                            // 持续 ticks 后全部关闭
                            for (i, &midi_note) in notes.iter().enumerate() {
                                if i == 0 {
                                    midi_track.add_event(ticks, note_off(channel, midi_note, 0));
                                } else {
                                    midi_track.add_event(0, note_off(channel, midi_note, 0));
                                }
                            }
                        }
                        MeasureEvent::Control(control) => {
                            match control {
                                LocalControl::LocalTempo(_) => {
                                    // 局部速度变化：当前版本不生成 tempo meta 事件
                                    // （需要绝对时间对齐到 tempo 轨道，后续版本支持）
                                }
                                LocalControl::LocalKey(_) | LocalControl::LocalTime(_) => {
                                    // 局部调号和拍号不影响 MIDI 音高
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    midi_track
}

#[cfg(test)]
mod tests {
    use super::*;
    use analysis::parser::parser::parse_score;

    fn sample_score() -> &'static str {
        r#"
@key(C major)
@tempo(120)
@time(4/4)
@title("Test Score")
track "PianoTrack" inst(piano) {
  section "Intro" {
      C4f4 D4f4 E4f4 F4f4 |
      C#4f4 Rf2 [G7]f4 [Cmaj7/E3]f4 |
  }
  section "Main" repeat(2) {
      E4f2. C4f4 G4f4 C5f1 |
  }
}
"#
    }

    #[test]
    fn test_score_to_midi_basic() {
        let score = parse_score(sample_score()).unwrap();
        let midi = score_to_midi(&score);
        // Format 1, 1 tempo track + 1 music track
        assert_eq!(midi.track_count(), 2);
        assert_eq!(midi.format, 1);
        assert_eq!(midi.division, 480);
    }

    #[test]
    fn test_midi_file_bytes_valid() {
        let score = parse_score(sample_score()).unwrap();
        let midi = score_to_midi(&score);
        let bytes = midi.to_bytes();

        // MThd header
        assert_eq!(&bytes[0..4], b"MThd");
        assert_eq!(&bytes[8..10], &[0, 1]); // format 1

        // 至少有 MThd + 2 个 MTrk
        assert!(bytes.len() > 14);

        // 检查是否包含 MTrk
        let mut mtrk_count = 0;
        for i in 0..bytes.len().saturating_sub(4) {
            if &bytes[i..i + 4] == b"MTrk" {
                mtrk_count += 1;
            }
        }
        assert_eq!(mtrk_count, 2);
    }

    #[test]
    fn test_note_count_in_midi() {
        let score = parse_score(sample_score()).unwrap();
        let midi = score_to_midi(&score);
        let bytes = midi.to_bytes();

        // 统计 Note On 事件 (0x90 | channel)
        let note_on_count = count_note_ons(&bytes);
        // 第一段: C D E F (4) + C# (1) + G7和弦(2音) + Cmaj7/E3(5音) = 12
        // 第二段重复2次: E C G C (4 * 2) = 8
        // 总计: 12 + 8 = 20
        assert_eq!(note_on_count, 20);
    }

    fn count_note_ons(bytes: &[u8]) -> usize {
        let mut count = 0;
        let mut i = 0;
        while i < bytes.len() {
            // 查找 Note On 事件 (0x90-0x9F)，且 velocity > 0
            if bytes[i] >= 0x90 && bytes[i] <= 0x9F {
                // 确保有后续两个字节
                if i + 2 < bytes.len() && bytes[i + 2] > 0 {
                    count += 1;
                    i += 3;
                    continue;
                }
            }
            i += 1;
        }
        count
    }

    #[test]
    fn test_duration_to_ticks() {
        let ppq = 480u16;

        // 四分音符 = 480 ticks
        let d4 = Duration::new(4, false);
        assert_eq!(duration_to_ticks(&d4, ppq), 480);

        // 二分音符 = 960 ticks
        let d2 = Duration::new(2, false);
        assert_eq!(duration_to_ticks(&d2, ppq), 960);

        // 附点二分音符 = 1440 ticks
        let d2d = Duration::new(2, true);
        assert_eq!(duration_to_ticks(&d2d, ppq), 1440);

        // 全音符 = 1920 ticks
        let d1 = Duration::new(1, false);
        assert_eq!(duration_to_ticks(&d1, ppq), 1920);
    }

    #[test]
    fn test_rest_advances_time() {
        let score = parse_score(
            r#"track "T" inst(piano) { section "S" { C4f4 Rf4 C4f4 | } }"#
        ).unwrap();
        let midi = score_to_midi(&score);
        let bytes = midi.to_bytes();

        // C4 休止 C4: 2个 Note On
        assert_eq!(count_note_ons(&bytes), 2);
    }

    #[test]
    fn test_multi_track() {
        let src = r#"
@tempo(120)
track "T1" inst(piano) { section "S1" { C4f4 | } }
track "T2" inst(piano) { section "S2" { E4f4 | } }
"#;
        let score = parse_score(src).unwrap();
        let midi = score_to_midi(&score);
        // 1 tempo track + 2 music tracks
        assert_eq!(midi.track_count(), 3);
    }
}
