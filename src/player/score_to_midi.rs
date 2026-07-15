//! Score → MIDI 事件转换器

use crate::analysis::kind::score::{Score, Track, MeasureEvent, LocalControl, PedalKind};
use crate::analysis::kind::note_value::Duration;

use crate::midi_writer::{MidiFile, MidiTrack, note_on, note_off, program_change, control_change, tempo_meta};

const DEFAULT_PPQ: u16 = 480;

fn duration_to_ticks(duration: &Duration, ppq: u16) -> u32 {
    let quarter_notes = duration.quarter_notes();
    (quarter_notes * ppq as f32).round() as u32
}

pub fn score_to_midi(score: &Score) -> MidiFile {
    score_to_midi_with_ppq(score, DEFAULT_PPQ)
}

pub fn score_to_midi_with_ppq(score: &Score, ppq: u16) -> MidiFile {
    let mut midi = MidiFile::new(1, ppq);

    let mut tempo_track = MidiTrack::new();
    tempo_track.set_name("Tempo");

    let bpm = score.global_bpm();
    let us_per_quarter = 60_000_000 / bpm.max(1) as u32;
    tempo_track.add_event(0, tempo_meta(us_per_quarter));

    midi.add_track(tempo_track);

    for (idx, track) in score.tracks.iter().enumerate() {
        let channel = (idx as u8).min(15);
        let midi_track = convert_track(track, channel, ppq);
        midi.add_track(midi_track);
    }

    midi
}

fn convert_track(track: &Track, channel: u8, ppq: u16) -> MidiTrack {
    let mut midi_track = MidiTrack::new();
    midi_track.set_name(&track.name);

    if let Some(ref inst) = track.instrument {
        midi_track.add_event(0, program_change(channel, inst.midi_program()));
        let (cc_vol, vol_val) = inst.midi_cc_volume();
        midi_track.add_event(0, control_change(channel, cc_vol, vol_val));
        let (cc_pan, pan_val) = inst.midi_cc_pan();
        midi_track.add_event(0, control_change(channel, cc_pan, pan_val));
    }

    for section in &track.sections {
        let repeat = section.repeat_times.unwrap_or(1);
        for _ in 0..repeat {
            for measure in &section.measures {
                for event in &measure.events {
                    match event {
                        MeasureEvent::Note(note) => {
                            let ticks = duration_to_ticks(&note.duration, ppq);
                            if note.is_rest() {
                                midi_track.advance(ticks);
                            } else {
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
                            for &midi_note in &notes {
                                midi_track.add_event(0, note_on(channel, midi_note, vel));
                            }
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
                                LocalControl::LocalTempo(_) => {}
                                LocalControl::LocalKey(_) | LocalControl::LocalTime(_) => {}
                                LocalControl::PedalOn(kind) => {
                                    let cc = pedal_cc(kind);
                                    midi_track.add_event(0, control_change(channel, cc, 127));
                                }
                                LocalControl::PedalOff(kind) => {
                                    let cc = pedal_cc(kind);
                                    midi_track.add_event(0, control_change(channel, cc, 0));
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

fn pedal_cc(kind: &PedalKind) -> u8 {
    match kind {
        PedalKind::Sustain => 64,
        PedalKind::Soft => 67,
        PedalKind::Sostenuto => 66,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::parser::parser::parse_score;

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
        assert_eq!(midi.track_count(), 2);
        assert_eq!(midi.format, 1);
        assert_eq!(midi.division, 480);
    }

    #[test]
    fn test_midi_file_bytes_valid() {
        let score = parse_score(sample_score()).unwrap();
        let midi = score_to_midi(&score);
        let bytes = midi.to_bytes();

        assert_eq!(&bytes[0..4], b"MThd");
        assert_eq!(&bytes[8..10], &[0, 1]);

        assert!(bytes.len() > 14);

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

        let note_on_count = count_note_ons(&bytes);
        assert_eq!(note_on_count, 20);
    }

    fn count_note_ons(bytes: &[u8]) -> usize {
        let mut count = 0;
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] >= 0x90 && bytes[i] <= 0x9F {
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

        let d4 = Duration::new(4, false);
        assert_eq!(duration_to_ticks(&d4, ppq), 480);

        let d2 = Duration::new(2, false);
        assert_eq!(duration_to_ticks(&d2, ppq), 960);

        let d2d = Duration::new(2, true);
        assert_eq!(duration_to_ticks(&d2d, ppq), 1440);

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
        assert_eq!(midi.track_count(), 3);
    }
}
