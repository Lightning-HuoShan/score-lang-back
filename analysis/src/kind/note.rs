use super::{ pitch::{ Pitch, MIDI_NOTE_MIN, MIDI_NOTE_MAX }, note_value::Duration };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteKind {
    Normal(Pitch),
    Rest,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub kind: NoteKind,
    pub duration: Duration,
    velocity: u8,
    pub track_id: usize,
}

impl Note {
    pub fn new_note(pitch: Pitch, duration: Duration, track_id: usize) -> Self {
        Self {
            kind: NoteKind::Normal(pitch),
            duration,
            velocity: 100,
            track_id,
        }
    }

    pub fn new_rest(duration: Duration, track_id: usize) -> Self {
        Self {
            kind: NoteKind::Rest,
            duration,
            velocity: 0,
            track_id,
        }
    }

    pub fn velocity(&self) -> u8 {
        self.velocity
    }

    pub fn set_velocity(&mut self, vel: u8) {
        self.velocity = vel.clamp(MIDI_NOTE_MIN, MIDI_NOTE_MAX);
    }

    pub fn midi_key(&self) -> Option<u8> {
        match self.kind {
            NoteKind::Normal(p) => p.midi_note(),
            NoteKind::Rest => None,
        }
    }

    pub fn midi_velocity(&self) -> u8 {
        self.velocity
    }

    pub fn midi_duration_us(&self, bpm: u16) -> u64 {
        self.duration.microseconds(bpm)
    }

    pub fn is_rest(&self) -> bool {
        matches!(self.kind, NoteKind::Rest)
    }

    /// 转调：按半音数偏移音高（休止符不受影响）
    pub fn transpose(&mut self, semitones: i8) {
        if let NoteKind::Normal(ref mut pitch) = self.kind {
            pitch.transpose(semitones);
        }
    }

    pub fn display(&self) -> String {
        let head = match self.kind {
            NoteKind::Normal(p) => p.display(),
            NoteKind::Rest => "R".to_string(),
        };
        format!("{}{}", head, self.duration.display())
    }
}