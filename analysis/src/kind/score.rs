use super::{
    chord::Chord,
    note::Note,
    key::Key,
    tempo::{Tempo, ConfigMarker},
    instrument::Instrument,
};

#[derive(Debug, Clone, PartialEq)]
pub struct TimeSig {
    pub beats_per_bar: u32,
    pub beat_value: u32,
    marker: ConfigMarker,
}

impl TimeSig {
    pub fn global(beats_per_bar: u32, beat_value: u32) -> Self {
        Self {
            beats_per_bar: beats_per_bar.max(1),
            beat_value: beat_value.max(1),
            marker: ConfigMarker::global(),
        }
    }

    pub fn local(beats_per_bar: u32, beat_value: u32) -> Self {
        Self {
            beats_per_bar: beats_per_bar.max(1),
            beat_value: beat_value.max(1),
            marker: ConfigMarker::local(),
        }
    }

    pub fn is_local(&self) -> bool {
        self.marker.is_local()
    }

    pub fn is_global(&self) -> bool {
        self.marker.is_global()
    }

    pub fn bar_total_value(&self) -> f32 {
        let beat_unit = 1.0 / self.beat_value as f32;
        beat_unit * self.beats_per_bar as f32
    }

    pub fn bar_quarter_notes(&self) -> f32 {
        self.bar_total_value() * 4.0
    }

    pub fn display(&self) -> String {
        format!("time({}/{})", self.beats_per_bar, self.beat_value)
    }
}

#[derive(Debug, Clone)]
pub enum LocalControl {
    LocalKey(Key),
    LocalTempo(Tempo),
    LocalTime(TimeSig),
}

#[derive(Debug, Clone)]
pub enum MeasureEvent {
    Note(Note),
    Chord(Chord),
    Control(LocalControl),
}

#[derive(Debug, Clone)]
pub struct Measure {
    pub events: Vec<MeasureEvent>,
    pub index: u32,
}

impl Measure {
    pub fn new(index: u32) -> Self { Self { events: Vec::new(), index } }
    pub fn push_event(&mut self, event: MeasureEvent) { self.events.push(event); }
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub repeat_times: Option<u32>,
    pub measures: Vec<Measure>,
}

impl Section {
    pub fn new(name: String) -> Self {
        Self { name, repeat_times: None, measures: Vec::new() }
    }
    pub fn set_repeat(&mut self, times: u32) { self.repeat_times = Some(times); }
    pub fn push_measure(&mut self, measure: Measure) { self.measures.push(measure); }
}

#[derive(Debug, Clone)]
pub struct Track {
    pub name: String,
    pub track_id: usize,
    pub instrument: Option<Instrument>,
    pub sections: Vec<Section>,
}

impl Track {
    pub fn new(name: String, track_id: usize) -> Self {
        Self { name, track_id, instrument: None, sections: Vec::new() }
    }
    pub fn set_instrument(&mut self, inst: Instrument) { self.instrument = Some(inst); }
    pub fn push_section(&mut self, sec: Section) { self.sections.push(sec); }
}

#[derive(Debug, Clone)]
pub struct Score {
    pub title: Option<String>,
    pub global_key: Option<Key>,
    pub global_tempo: Option<Tempo>,
    pub global_time: Option<TimeSig>,
    pub tracks: Vec<Track>,
}

impl Score {
    pub fn empty() -> Self {
        Self {
            title: None,
            global_key: None,
            global_tempo: None,
            global_time: None,
            tracks: Vec::new(),
        }
    }
    pub fn set_title(&mut self, title: String) { self.title = Some(title); }
    pub fn set_global_key(&mut self, key: Key) { self.global_key = Some(key); }
    pub fn set_global_tempo(&mut self, tempo: Tempo) { self.global_tempo = Some(tempo); }
    pub fn set_global_time(&mut self, sig: TimeSig) { self.global_time = Some(sig); }
    pub fn push_track(&mut self, track: Track) { self.tracks.push(track); }

    pub fn all_notes<'a>(&'a self) -> Vec<&'a Note> {
        let mut notes = Vec::new();
        for track in &self.tracks {
            for section in &track.sections {
                let repeat = section.repeat_times.unwrap_or(1);
                for _ in 0..repeat {
                    for measure in &section.measures {
                        for event in &measure.events {
                            if let MeasureEvent::Note(note) = event {
                                notes.push(note);
                            }
                        }
                    }
                }
            }
        }
        notes
    }

    pub fn all_chords<'a>(&'a self) -> Vec<&'a Chord> {
        let mut chords = Vec::new();
        for track in &self.tracks {
            for section in &track.sections {
                let repeat = section.repeat_times.unwrap_or(1);
                for _ in 0..repeat {
                    for measure in &section.measures {
                        for event in &measure.events {
                            if let MeasureEvent::Chord(chord) = event {
                                chords.push(chord);
                            }
                        }
                    }
                }
            }
        }
        chords
    }

    pub fn global_bpm(&self) -> u16 {
        self.global_tempo.as_ref().map(|t| t.bpm()).unwrap_or(120)
    }

    /// 整体转调：将所有音符/和弦按半音数偏移
    ///
    /// 正数=升调，负数=降调
    pub fn transpose(&mut self, semitones: i8) {
        // 转调全局调号
        if let Some(ref mut key) = self.global_key {
            *key = key.transpose(semitones);
        }

        // 转调所有音符
        for track in &mut self.tracks {
            for section in &mut track.sections {
                for measure in &mut section.measures {
                    for event in &mut measure.events {
                        match event {
                            MeasureEvent::Note(note) => {
                                note.transpose(semitones);
                            }
                            MeasureEvent::Chord(chord) => {
                                chord.transpose(semitones);
                            }
                            MeasureEvent::Control(_) => {}
                        }
                    }
                }
            }
        }
    }
}
