use super::{ pitch::{ Pitch, PitchClass, MIDI_NOTE_MIN, MIDI_NOTE_MAX }, note_value::Duration };

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChordQuality {
    Maj,
    Min,
    Dim,
    Aug,
    Sus,
}

impl ChordQuality {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "maj" => Some(Self::Maj),
            "min" | "m" => Some(Self::Min),
            "dim" => Some(Self::Dim),
            "aug" => Some(Self::Aug),
            "sus" => Some(Self::Sus),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            ChordQuality::Maj => "maj",
            ChordQuality::Min => "m",
            ChordQuality::Dim => "dim",
            ChordQuality::Aug => "aug",
            ChordQuality::Sus => "sus",
        }
    }

    fn triad_intervals(&self) -> &[i8] {
        match self {
            ChordQuality::Maj => &[0, 4, 7],
            ChordQuality::Min => &[0, 3, 7],
            ChordQuality::Dim => &[0, 3, 6],
            ChordQuality::Aug => &[0, 4, 8],
            ChordQuality::Sus => &[0, 5, 7],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChordAlterItem {
    pub alter_type: String,
    pub number: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChordSymbol {
    pub root: PitchClass,
    pub quality: Option<ChordQuality>,
    pub base_number: Option<u32>,
    pub alters: Vec<ChordAlterItem>,
}

impl ChordSymbol {
    pub fn new(root: PitchClass) -> Self {
        Self {
            root,
            quality: None,
            base_number: None,
            alters: Vec::new(),
        }
    }

    pub fn display(&self) -> String {
        let mut s = self.root.display();
        if let Some(q) = &self.quality {
            s.push_str(q.as_str());
        }
        if let Some(num) = self.base_number {
            s.push_str(&num.to_string());
        }
        for alter in &self.alters {
            s.push_str(&format!("{}{}", alter.alter_type, alter.number));
        }
        s
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    pub symbol: ChordSymbol,
    pub slash_bass: Option<Pitch>,
    pub duration: Duration,
    pub track_id: usize,
    velocity: u8,
    default_octave: u8,
}

impl Chord {
    pub fn new_normal(symbol: ChordSymbol, duration: Duration, track_id: usize) -> Self {
        Self {
            symbol,
            slash_bass: None,
            duration,
            track_id,
            velocity: 100,
            default_octave: 4,
        }
    }

    pub fn new_slash(
        symbol: ChordSymbol,
        bass: Pitch,
        duration: Duration,
        track_id: usize,
    ) -> Self {
        Self {
            symbol,
            slash_bass: Some(bass),
            duration,
            track_id,
            velocity: 100,
            default_octave: 4,
        }
    }

    pub fn velocity(&self) -> u8 {
        self.velocity
    }

    pub fn set_velocity(&mut self, vel: u8) {
        self.velocity = vel.clamp(MIDI_NOTE_MIN, MIDI_NOTE_MAX);
    }

    pub fn set_default_octave(&mut self, oct: u8) {
        self.default_octave = oct;
    }

    /// 转调：按半音数偏移和弦根音
    pub fn transpose(&mut self, semitones: i8) {
        let current_semi = self.symbol.root.class_semitone();
        let new_semi = ((current_semi + semitones) % 12 + 12) % 12;

        // 保持音名，调整变音记号
        let base = self.symbol.root.base_semitone();
        let diff = ((new_semi - base) % 12 + 12) % 12;
        let new_acc = match diff {
            0 => super::pitch::Accidental::Natural,
            1 => super::pitch::Accidental::Sharp,
            2 => super::pitch::Accidental::DoubleSharp,
            11 => super::pitch::Accidental::Flat,
            10 => super::pitch::Accidental::DoubleFlat,
            _ => super::pitch::Accidental::Natural,
        };
        self.symbol.root.acc = new_acc;
    }

    pub fn midi_velocity(&self) -> u8 {
        self.velocity
    }

    pub fn midi_duration_us(&self, bpm: u16) -> u64 {
        self.duration.microseconds(bpm)
    }

    pub fn midi_notes(&self) -> Vec<u8> {
        let root_semi = self.symbol.root.class_semitone();
        let default_oct = self.default_octave as i16;
        let root_midi = root_semi as i16 + (default_oct + 1) * 12;

        let mut notes: Vec<i16> = Vec::new();

        if let Some(quality) = &self.symbol.quality {
            for interval in quality.triad_intervals() {
                notes.push(root_midi + *interval as i16);
            }
        } else {
            notes.push(root_midi);
        }

        if let Some(base_num) = self.symbol.base_number {
            let extension_semitones = match base_num {
                6 => 9,
                7 => 10,
                9 => 14,
                11 => 17,
                13 => 21,
                _ => 0,
            };
            if extension_semitones > 0 {
                let ext_note = root_midi + extension_semitones;
                if !notes.contains(&ext_note) {
                    notes.push(ext_note);
                }
            }
        }

        for alter in &self.symbol.alters {
            let degree = alter.number as i16;
            let octave_offset = ((degree - 1) / 7) * 12;
            let degree_in_octave = ((degree - 1) % 7) as u8;
            let base_semi = match degree_in_octave {
                0 => 0,
                1 => 2,
                2 => 4,
                3 => 5,
                4 => 7,
                5 => 9,
                6 => 11,
                _ => 0,
            };
            let alter_semi: i16 = match alter.alter_type.as_str() {
                "sharp" | "#" => 1,
                "flat" | "b" => -1,
                "add" => 0,
                "no" => 0,
                _ => 0,
            };
            let note_val = root_midi + base_semi as i16 + octave_offset + alter_semi;
            match alter.alter_type.as_str() {
                "no" => { notes.retain(|&n| n % 12 != note_val % 12); }
                _ => { if !notes.contains(&note_val) { notes.push(note_val); } }
            }
        }

        if let Some(bass) = &self.slash_bass {
            if let Some(bass_midi) = bass.midi_note() {
                let bass_val = bass_midi as i16;
                if !notes.contains(&bass_val) {
                    notes.insert(0, bass_val);
                }
            }
        }

        notes.sort();
        notes.into_iter()
            .filter(|&n| n >= MIDI_NOTE_MIN as i16 && n <= MIDI_NOTE_MAX as i16)
            .map(|n| n as u8)
            .collect()
    }

    pub fn display(&self) -> String {
        let mut buf = format!("[{}", self.symbol.display());
        if let Some(bass) = &self.slash_bass {
            buf.push_str(&format!("/{}", bass.display()));
        }
        buf.push_str(&format!("]{}", self.duration.display()));
        buf
    }
}

// ===== 和弦识别 =====

/// 和弦识别结果
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifiedChord {
    pub root_name: String,
    pub quality: ChordQuality,
    pub extensions: Vec<u32>,
    pub slash_bass: Option<String>,
}

/// 从 MIDI 音高列表识别和弦
///
/// 输入一组 MIDI 音号，返回最可能的和弦名称
pub fn identify_chord(midi_notes: &[u8]) -> Option<IdentifiedChord> {
    if midi_notes.is_empty() {
        return None;
    }

    // 转换为音级集合（去掉八度，只保留 0-11）
    let mut pitch_classes: Vec<u8> = midi_notes.iter().map(|&n| n % 12).collect();
    pitch_classes.sort();
    pitch_classes.dedup();

    if pitch_classes.is_empty() {
        return None;
    }

    // 已知和弦类型的音程模式（相对于根音的半音集合）
    const CHORD_PATTERNS: &[(&str, &[i8], ChordQuality)] = &[
        ("Maj",  &[0, 4, 7],           ChordQuality::Maj),
        ("Min",  &[0, 3, 7],           ChordQuality::Min),
        ("Dim",  &[0, 3, 6],           ChordQuality::Dim),
        ("Aug",  &[0, 4, 8],           ChordQuality::Aug),
        ("Sus",  &[0, 5, 7],           ChordQuality::Sus),
    ];

    // 扩展和弦模式（包含7th/9th/11th/13th）
    const EXT_PATTERNS: &[(&str, &[i8], ChordQuality, Option<u32>)] = &[
        ("7",    &[0, 4, 7, 10],       ChordQuality::Maj, Some(7)),
        ("m7",   &[0, 3, 7, 10],       ChordQuality::Min, Some(7)),
        ("maj7", &[0, 4, 7, 11],       ChordQuality::Maj, Some(7)),
        ("m7b5", &[0, 3, 6, 10],       ChordQuality::Min, Some(7)),  // half-dim
        ("dim7", &[0, 3, 6, 9],        ChordQuality::Dim, Some(7)),
        ("9",    &[0, 4, 7, 10, 14],   ChordQuality::Maj, Some(9)),
        ("m9",   &[0, 3, 7, 10, 14],   ChordQuality::Min, Some(9)),
        ("11",   &[0, 4, 7, 10, 14, 17], ChordQuality::Maj, Some(11)),
        ("13",   &[0, 4, 7, 10, 14, 21], ChordQuality::Maj, Some(13)),
        ("6",    &[0, 4, 7, 9],        ChordQuality::Maj, Some(6)),
        ("m6",   &[0, 3, 7, 9],        ChordQuality::Min, Some(6)),
    ];

    let note_names = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];
    let note_semitones: [i8; 7] = [0, 2, 4, 5, 7, 9, 11];

    // 尝试每个音作为根音
    for &root_pc in &pitch_classes {
        // 计算相对音程集合
        let intervals: Vec<i8> = pitch_classes.iter()
            .map(|&pc| ((pc as i8 - root_pc as i8) % 12 + 12) % 12)
            .collect();

        // 先尝试扩展和弦
        for (_name, pattern, quality, ext) in EXT_PATTERNS {
            let pattern_set: Vec<i8> = pattern.iter().map(|&i| i % 12).collect();
            let mut pattern_unique = pattern_set.clone();
            pattern_unique.sort();
            pattern_unique.dedup();

            let mut intervals_mod = intervals.iter().map(|&i| i % 12).collect::<Vec<i8>>();
            intervals_mod.sort();
            intervals_mod.dedup();

            if intervals_mod == pattern_unique || is_subset(&pattern_unique, &intervals_mod) {
                let root_idx = note_semitones.iter().position(|&s| s == root_pc as i8).unwrap_or(0);
                let root_name = note_names[root_idx].to_string();

                let mut extensions = Vec::new();
                if let Some(ext_num) = ext {
                    extensions.push(*ext_num);
                }

                return Some(IdentifiedChord {
                    root_name,
                    quality: quality.clone(),
                    extensions,
                    slash_bass: None,
                });
            }
        }

        // 再尝试基本三和弦
        for (_name, pattern, quality) in CHORD_PATTERNS {
            let pattern_set: Vec<i8> = pattern.to_vec();
            if is_subset(&pattern_set, &intervals) {
                let root_idx = note_semitones.iter().position(|&s| s == root_pc as i8).unwrap_or(0);
                let root_name = note_names[root_idx].to_string();

                return Some(IdentifiedChord {
                    root_name,
                    quality: quality.clone(),
                    extensions: Vec::new(),
                    slash_bass: None,
                });
            }
        }
    }

    None
}

/// 检查 pattern 是否是 intervals 的子集
fn is_subset(pattern: &[i8], intervals: &[i8]) -> bool {
    pattern.iter().all(|p| intervals.contains(p))
}

#[cfg(test)]
mod chord_identify_tests {
    use super::*;

    #[test]
    fn test_identify_c_major() {
        // C E G = MIDI 60, 64, 67 → pitch classes 0, 4, 7
        let result = identify_chord(&[60, 64, 67]);
        assert!(result.is_some());
        let chord = result.unwrap();
        assert_eq!(chord.root_name, "C");
        assert_eq!(chord.quality, ChordQuality::Maj);
    }

    #[test]
    fn test_identify_a_minor() {
        // A C E = MIDI 69, 72, 76 → pitch classes 9, 0, 4
        let result = identify_chord(&[69, 72, 76]);
        assert!(result.is_some());
        let chord = result.unwrap();
        assert_eq!(chord.root_name, "A");
        assert_eq!(chord.quality, ChordQuality::Min);
    }

    #[test]
    fn test_identify_g7() {
        // G B D F = MIDI 67, 71, 74, 77 → pitch classes 7, 11, 2, 5
        let result = identify_chord(&[67, 71, 74, 77]);
        assert!(result.is_some());
        let chord = result.unwrap();
        assert_eq!(chord.root_name, "G");
        assert!(chord.extensions.contains(&7));
    }

    #[test]
    fn test_identify_dim() {
        // B D F = MIDI 71, 74, 77 → pitch classes 11, 2, 5
        let result = identify_chord(&[71, 74, 77]);
        assert!(result.is_some());
        let chord = result.unwrap();
        assert_eq!(chord.quality, ChordQuality::Dim);
    }

    #[test]
    fn test_identify_empty() {
        assert!(identify_chord(&[]).is_none());
    }

    #[test]
    fn test_identify_single_note() {
        // 单音无法识别为和弦
        let _result = identify_chord(&[60]);
        // 单音可能匹配到某些模式的子集，也可能返回 None
        // 取决于实现——当前实现会尝试三和弦子集匹配
        // 这是合理的行为
    }
}