pub const MIDI_NOTE_MIN: u8 = 0;
pub const MIDI_NOTE_MAX: u8 = 127;
pub const MIDI_A4: u8 = 69;
pub const MIDI_FREQ_A4: f64 = 440.0;

/// 变音记号，对应语法 Accidental: # | b | bb | x | =
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accidental {
    Natural,
    Sharp,
    DoubleSharp,
    Flat,
    DoubleFlat,
}

impl Accidental {
    /// 从Lexer读取的字符串转换变音记号
    pub fn from_str(s: &str) -> Self {
        match s {
            "=" => Self::Natural,
            "#" => Self::Sharp,
            "x" => Self::DoubleSharp,
            "b" => Self::Flat,
            "bb" => Self::DoubleFlat,
            _ => Self::Natural,
        }
    }

    /// 获取该变音记号的半音偏移量
    pub fn semitone_offset(&self) -> i8 {
        match self {
            Accidental::Natural => 0,
            Accidental::Sharp => 1,
            Accidental::DoubleSharp => 2,
            Accidental::Flat => -1,
            Accidental::DoubleFlat => -2,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Accidental::Natural => "=",
            Accidental::Sharp => "#",
            Accidental::DoubleSharp => "x",
            Accidental::Flat => "b",
            Accidental::DoubleFlat => "bb",
        }
    }
}

/// 音级：音名+变音，无八度（用于和弦根音）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PitchClass {
    pub name: char,
    pub acc: Accidental,
}

impl PitchClass {
    pub fn new(name: char, acc: Accidental) -> Self {
        Self { name, acc }
    }

    /// 获取基础音名的半音基准（C=0 ~ B=11）
    pub fn base_semitone(&self) -> i8 {
        match self.name {
            'C' => 0,
            'D' => 2,
            'E' => 4,
            'F' => 5,
            'G' => 7,
            'A' => 9,
            'B' => 11,
            _ => 0, // 非法音名兜底C
        }
    }

    /// 仅音级自身总半音偏移（不含八度）
    pub fn class_semitone(&self) -> i8 {
        self.base_semitone() + self.acc.semitone_offset()
    }

    pub fn display(&self) -> String {
        let mut buf = String::new();
        buf.push(self.name);
        buf.push_str(self.acc.to_string());
        buf
    }
}

/// 完整音高载体：音级 + 可选八度，内置MIDI音号计算
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pitch {
    pub class: PitchClass,
    /// None = 和弦根音，无八度，无法计算MIDI
    pub octave: Option<u8>,
}

impl Pitch {
    /// 带八度，可计算MIDI音高（单音符使用）
    pub fn with_octave(name: char, acc: Accidental, octave: u8) -> Self {
        Self {
            class: PitchClass::new(name, acc),
            octave: Some(octave),
        }
    }

    /// 无八度，和弦符号专用，无MIDI数值
    pub fn without_octave(name: char, acc: Accidental) -> Self {
        Self {
            class: PitchClass::new(name, acc),
            octave: None,
        }
    }

    pub fn midi_note(&self) -> Option<u8> {
        let oct = self.octave? as i16;
        let base_total = self.class.class_semitone() as i16 + (oct + 1) * 12;
        if base_total >= MIDI_NOTE_MIN as i16 && base_total <= MIDI_NOTE_MAX as i16 {
            Some(base_total as u8)
        } else {
            None
        }
    }

    pub fn midi_frequency(&self) -> Option<f64> {
        let midi = self.midi_note()? as i16;
        let semitones_from_a4 = midi - MIDI_A4 as i16;
        let freq = MIDI_FREQ_A4 * (2.0f64).powf(semitones_from_a4 as f64 / 12.0);
        Some(freq)
    }

    /// 转调：按半音数偏移，保持原有音名体系
    pub fn transpose(&mut self, semitones: i8) {
        if let Some(oct) = self.octave {
            let current_midi = self.class.class_semitone() as i16 + (oct as i16 + 1) * 12;
            let new_midi = current_midi + semitones as i16;
            if new_midi >= 0 && new_midi <= 127 {
                let new_oct = (new_midi / 12 - 1) as u8;
                let new_pc_semi = (new_midi % 12) as i8;
                // 保持音名，调整变音记号
                let base = self.class.base_semitone();
                let diff = ((new_pc_semi - base) % 12 + 12) % 12;
                let new_acc = match diff {
                    0 => Accidental::Natural,
                    1 => Accidental::Sharp,
                    2 => Accidental::DoubleSharp,
                    11 => Accidental::Flat,
                    10 => Accidental::DoubleFlat,
                    _ => Accidental::Natural,
                };
                self.class.acc = new_acc;
                self.octave = Some(new_oct);
            }
        }
    }

    pub fn display(&self) -> String {
        let mut s = self.class.display();
        if let Some(oct) = self.octave {
            s.push_str(&oct.to_string());
        }
        s
    }
}
