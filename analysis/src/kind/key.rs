use super::pitch::{PitchClass, Accidental};
use super::tempo::ConfigMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyMode {
    Major,
    Minor,
    HarmonicMinor,
    MelodicMinor,
}

impl KeyMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "major" => Some(Self::Major),
            "minor" | "natural_minor" => Some(Self::Minor),
            "harmonic_minor" => Some(Self::HarmonicMinor),
            "melodic_minor" => Some(Self::MelodicMinor),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            KeyMode::Major => "major",
            KeyMode::Minor => "minor",
            KeyMode::HarmonicMinor => "harmonic_minor",
            KeyMode::MelodicMinor => "melodic_minor",
        }
    }

    /// 返回该调式相对于根音的半音间隔模式
    fn interval_pattern(&self) -> &'static [i8] {
        match self {
            KeyMode::Major => &[0, 2, 4, 5, 7, 9, 11],
            KeyMode::Minor => &[0, 2, 3, 5, 7, 8, 10],
            KeyMode::HarmonicMinor => &[0, 2, 3, 5, 7, 8, 11],
            KeyMode::MelodicMinor => &[0, 2, 3, 5, 7, 9, 11],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Key {
    pub root: PitchClass,
    pub mode: KeyMode,
    marker: ConfigMarker,
}

impl Key {
    pub fn global(root: PitchClass, mode: KeyMode) -> Self {
        Self {
            root,
            mode,
            marker: ConfigMarker::global(),
        }
    }

    pub fn local(root: PitchClass, mode: KeyMode) -> Self {
        Self {
            root,
            mode,
            marker: ConfigMarker::local(),
        }
    }

    pub fn root_name(&self) -> String {
        self.root.display()
    }

    pub fn is_major(&self) -> bool {
        matches!(self.mode, KeyMode::Major)
    }

    pub fn is_minor(&self) -> bool {
        matches!(self.mode, KeyMode::Minor | KeyMode::HarmonicMinor | KeyMode::MelodicMinor)
    }

    pub fn is_local(&self) -> bool {
        self.marker.is_local()
    }

    pub fn is_global(&self) -> bool {
        self.marker.is_global()
    }

    pub fn root_semitone(&self) -> i8 {
        self.root.class_semitone()
    }

    /// 生成该调的音阶（7 个音级），每个音级用 PitchClass 表示
    ///
    /// 使用五度圈和调号规则确保音名不重复（每个音级用不同的字母名）
    pub fn scale(&self) -> [PitchClass; 7] {
        let root_semi = self.root.class_semitone();
        let pattern = self.mode.interval_pattern();
        let mut result = [PitchClass::new('C', Accidental::Natural); 7];

        // 音级字母名序列：从根音字母名开始
        let root_name = self.root.name.to_ascii_uppercase();
        let name_sequence = letter_sequence_from(root_name);

        for i in 0..7 {
            let target_semi = ((root_semi + pattern[i]) % 12 + 12) % 12;
            let name = name_sequence[i];
            result[i] = pitch_class_for_target(name, target_semi);
        }

        result
    }

    /// 返回该调号需要的变音记号列表
    ///
    /// 例如 C major 返回空, G major 返回 [F#], D major 返回 [F#, C#]
    pub fn accidentals(&self) -> Vec<PitchClass> {
        let scale = self.scale();
        let natural_semitones = [0, 2, 4, 5, 7, 9, 11]; // C major
        let natural_names = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];

        let mut result = Vec::new();
        for i in 0..7 {
            let actual_semi = scale[i].class_semitone();
            let expected_semi = natural_semitones[i];
            if actual_semi != expected_semi || scale[i].name != natural_names[i] {
                // 该音级需要变音记号
                if scale[i].acc != Accidental::Natural {
                    result.push(scale[i]);
                }
            }
        }
        result
    }

    /// 判断给定音级在该调中是否需要变音记号
    pub fn accidental_for_degree(&self, degree: u8) -> Option<Accidental> {
        if degree == 0 || degree > 7 {
            return None;
        }
        let scale = self.scale();
        Some(scale[degree as usize - 1].acc)
    }

    /// 调号中的升降号数量（正数=升号，负数=降号）
    ///
    /// 例如 C major=0, G major=1, F major=-1
    pub fn accidental_count(&self) -> i8 {
        let scale = self.scale();
        let mut count: i8 = 0;
        for pc in &scale {
            match pc.acc {
                Accidental::Sharp | Accidental::DoubleSharp => count += 1,
                Accidental::Flat | Accidental::DoubleFlat => count -= 1,
                Accidental::Natural => {}
            }
        }
        count
    }

    pub fn transpose(&self, semitones: i8) -> Self {
        let root_semi = self.root.class_semitone();
        let new_semi = ((root_semi + semitones) % 12 + 12) % 12;
        let (new_name, new_acc_offset) = match new_semi {
            0 => ('C', 0),
            1 => ('C', 1),
            2 => ('D', 0),
            3 => ('D', 1),
            4 => ('E', 0),
            5 => ('F', 0),
            6 => ('F', 1),
            7 => ('G', 0),
            8 => ('G', 1),
            9 => ('A', 0),
            10 => ('A', 1),
            11 => ('B', 0),
            _ => ('C', 0),
        };
        use super::pitch::Accidental;
        let new_acc = match new_acc_offset {
            -2 => Accidental::DoubleFlat,
            -1 => Accidental::Flat,
            0 => Accidental::Natural,
            1 => Accidental::Sharp,
            2 => Accidental::DoubleSharp,
            _ => Accidental::Natural,
        };
        let new_root = PitchClass::new(new_name, new_acc);
        Self {
            root: new_root,
            mode: self.mode,
            marker: self.marker,
        }
    }

    pub fn display(&self) -> String {
        format!("key({} {})", self.root.display(), self.mode.as_str())
    }
}

/// 从根音字母名开始，生成 7 个不重复的字母名序列
fn letter_sequence_from(root: char) -> [char; 7] {
    const NAMES: [char; 7] = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];
    let start = NAMES.iter().position(|&n| n == root).unwrap_or(0);
    let mut result = ['C'; 7];
    for i in 0..7 {
        result[i] = NAMES[(start + i) % 7];
    }
    result
}

/// 给定目标字母名和目标半音值，计算需要的变音记号
fn pitch_class_for_target(name: char, target_semi: i8) -> PitchClass {
    let base = match name {
        'C' => 0, 'D' => 2, 'E' => 4, 'F' => 5, 'G' => 7, 'A' => 9, 'B' => 11,
        _ => 0,
    };
    let diff = ((target_semi - base) % 12 + 12) % 12;
    let acc = match diff {
        0 => Accidental::Natural,
        1 => Accidental::Sharp,
        2 => Accidental::DoubleSharp,
        11 => Accidental::Flat,
        10 => Accidental::DoubleFlat,
        _ => Accidental::Natural, // 不应该出现
    };
    PitchClass::new(name, acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pc(name: char, acc: Accidental) -> PitchClass {
        PitchClass::new(name, acc)
    }

    #[test]
    fn test_c_major_scale() {
        let key = Key::global(pc('C', Accidental::Natural), KeyMode::Major);
        let scale = key.scale();
        assert_eq!(scale[0], pc('C', Accidental::Natural));
        assert_eq!(scale[2], pc('E', Accidental::Natural));
        assert_eq!(scale[6], pc('B', Accidental::Natural));
    }

    #[test]
    fn test_g_major_scale() {
        let key = Key::global(pc('G', Accidental::Natural), KeyMode::Major);
        let scale = key.scale();
        assert_eq!(scale[0], pc('G', Accidental::Natural));
        assert_eq!(scale[6], pc('F', Accidental::Sharp)); // F#
    }

    #[test]
    fn test_d_major_scale() {
        let key = Key::global(pc('D', Accidental::Natural), KeyMode::Major);
        let scale = key.scale();
        assert_eq!(scale[0], pc('D', Accidental::Natural));
        assert_eq!(scale[2], pc('F', Accidental::Sharp));
        assert_eq!(scale[6], pc('C', Accidental::Sharp));
    }

    #[test]
    fn test_f_major_scale() {
        let key = Key::global(pc('F', Accidental::Natural), KeyMode::Major);
        let scale = key.scale();
        assert_eq!(scale[0], pc('F', Accidental::Natural));
        assert_eq!(scale[3], pc('B', Accidental::Flat)); // Bb
    }

    #[test]
    fn test_a_minor_scale() {
        let key = Key::global(pc('A', Accidental::Natural), KeyMode::Minor);
        let scale = key.scale();
        assert_eq!(scale[0], pc('A', Accidental::Natural));
        assert_eq!(scale[2], pc('C', Accidental::Natural));
        assert_eq!(scale[5], pc('F', Accidental::Natural));
    }

    #[test]
    fn test_a_harmonic_minor() {
        let key = Key::global(pc('A', Accidental::Natural), KeyMode::HarmonicMinor);
        let scale = key.scale();
        assert_eq!(scale[6], pc('G', Accidental::Sharp)); // G#
    }

    #[test]
    fn test_accidentals_g_major() {
        let key = Key::global(pc('G', Accidental::Natural), KeyMode::Major);
        let accs = key.accidentals();
        assert_eq!(accs.len(), 1);
        assert_eq!(accs[0], pc('F', Accidental::Sharp));
    }

    #[test]
    fn test_accidentals_c_major() {
        let key = Key::global(pc('C', Accidental::Natural), KeyMode::Major);
        assert!(key.accidentals().is_empty());
    }

    #[test]
    fn test_accidental_count() {
        assert_eq!(Key::global(pc('C', Accidental::Natural), KeyMode::Major).accidental_count(), 0);
        assert_eq!(Key::global(pc('G', Accidental::Natural), KeyMode::Major).accidental_count(), 1);
        assert_eq!(Key::global(pc('F', Accidental::Natural), KeyMode::Major).accidental_count(), -1);
    }

    #[test]
    fn test_accidental_for_degree() {
        let key = Key::global(pc('G', Accidental::Natural), KeyMode::Major);
        assert_eq!(key.accidental_for_degree(7), Some(Accidental::Sharp)); // F#
    }

    #[test]
    fn test_letter_sequence() {
        assert_eq!(letter_sequence_from('C'), ['C', 'D', 'E', 'F', 'G', 'A', 'B']);
        assert_eq!(letter_sequence_from('G'), ['G', 'A', 'B', 'C', 'D', 'E', 'F']);
    }
}
