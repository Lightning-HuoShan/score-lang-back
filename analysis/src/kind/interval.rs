//! 音程计算模块

use super::pitch::{Pitch, PitchClass};

/// 音程质量
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntervalQuality {
    Perfect,       // 纯
    Major,         // 大
    Minor,         // 小
    Augmented,     // 增
    Diminished,    // 减
    DoublyAugmented,
    DoublyDiminished,
}

impl IntervalQuality {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Perfect => "P",
            Self::Major => "M",
            Self::Minor => "m",
            Self::Augmented => "A",
            Self::Diminished => "d",
            Self::DoublyAugmented => "AA",
            Self::DoublyDiminished => "dd",
        }
    }
}

/// 音程：由音程度数和质量组成
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    /// 音程度数（1=一度, 2=二度, ..., 8=八度）
    pub degree: u8,
    pub quality: IntervalQuality,
}

impl Interval {
    pub fn new(degree: u8, quality: IntervalQuality) -> Self {
        Self { degree, quality }
    }

    /// 音程的半音数
    pub fn semitones(&self) -> i8 {
        let base = match self.degree {
            1 => 0,
            2 => 2,
            3 => 4,
            4 => 5,
            5 => 7,
            6 => 9,
            7 => 11,
            8 => 12,
            _ => 0,
        };
        let offset = match self.quality {
            IntervalQuality::Perfect => 0,
            IntervalQuality::Major => 0,
            IntervalQuality::Minor => -1,
            IntervalQuality::Augmented => 1,
            IntervalQuality::Diminished => -1,
            IntervalQuality::DoublyAugmented => 2,
            IntervalQuality::DoublyDiminished => -2,
        };
        // 纯音程只有 P4/P5/P8(和 P1)，其他度数用大小
        base + offset
    }

    /// 从半音数和度数推断音程
    pub fn from_semitones(semitones: i8, degree: u8) -> Self {
        let normalized = ((semitones % 12) + 12) % 12;
        let base = match degree {
            1 => 0,
            2 => 2,
            3 => 4,
            4 => 5,
            5 => 7,
            6 => 9,
            7 => 11,
            8 => 12,
            _ => 0,
        };
        let diff = normalized - base;

        let is_perfect_degree = matches!(degree, 1 | 4 | 5 | 8);
        let quality = if is_perfect_degree {
            match diff {
                -2 => IntervalQuality::DoublyDiminished,
                -1 => IntervalQuality::Diminished,
                0 => IntervalQuality::Perfect,
                1 => IntervalQuality::Augmented,
                2 => IntervalQuality::DoublyAugmented,
                _ => IntervalQuality::Perfect,
            }
        } else {
            match diff {
                ..=-3 => IntervalQuality::DoublyDiminished,
                -2 => IntervalQuality::Diminished,
                -1 => IntervalQuality::Minor,
                0 => IntervalQuality::Major,
                1 => IntervalQuality::Augmented,
                2.. => IntervalQuality::DoublyAugmented,
            }
        };
        Self { degree, quality }
    }

    pub fn display(&self) -> String {
        format!("{}{}", self.quality.as_str(), self.degree)
    }

    /// 常用音程快捷构造
    pub fn unison() -> Self { Self::new(1, IntervalQuality::Perfect) }
    pub fn minor_second() -> Self { Self::new(2, IntervalQuality::Minor) }
    pub fn major_second() -> Self { Self::new(2, IntervalQuality::Major) }
    pub fn minor_third() -> Self { Self::new(3, IntervalQuality::Minor) }
    pub fn major_third() -> Self { Self::new(3, IntervalQuality::Major) }
    pub fn perfect_fourth() -> Self { Self::new(4, IntervalQuality::Perfect) }
    pub fn tritone() -> Self { Self::new(4, IntervalQuality::Augmented) }
    pub fn perfect_fifth() -> Self { Self::new(5, IntervalQuality::Perfect) }
    pub fn minor_sixth() -> Self { Self::new(6, IntervalQuality::Minor) }
    pub fn major_sixth() -> Self { Self::new(6, IntervalQuality::Major) }
    pub fn minor_seventh() -> Self { Self::new(7, IntervalQuality::Minor) }
    pub fn major_seventh() -> Self { Self::new(7, IntervalQuality::Major) }
    pub fn octave() -> Self { Self::new(8, IntervalQuality::Perfect) }
}

/// 计算两个 PitchClass 之间的音程
pub fn interval_between(a: PitchClass, b: PitchClass) -> Interval {
    let letter_names = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];
    let a_pos = letter_names.iter().position(|&n| n == a.name).unwrap_or(0) as i8;
    let b_pos = letter_names.iter().position(|&n| n == b.name).unwrap_or(0) as i8;

    let degree = ((b_pos - a_pos + 7) % 7 + 1) as u8;
    let semi_diff = ((b.class_semitone() - a.class_semitone()) % 12 + 12) % 12;

    Interval::from_semitones(semi_diff, degree)
}

/// 计算两个完整音高（含八度）之间的音程
pub fn interval_between_pitches(a: &Pitch, b: &Pitch) -> Interval {
    let letter_names = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];
    let a_oct = a.octave.unwrap_or(4) as i16;
    let b_oct = b.octave.unwrap_or(4) as i16;
    let a_pos = letter_names.iter().position(|&n| n == a.class.name).unwrap_or(0) as i16;
    let b_pos = letter_names.iter().position(|&n| n == b.class.name).unwrap_or(0) as i16;

    let total_letter_steps = b_pos - a_pos + (b_oct - a_oct) * 7;
    let simple_degree = ((total_letter_steps % 7 + 7) % 7 + 1) as u8;

    let a_midi = a.class.class_semitone() as i16 + (a_oct + 1) * 12;
    let b_midi = b.class.class_semitone() as i16 + (b_oct + 1) * 12;
    let semi_diff = (b_midi - a_midi) as i8;
    let normalized = ((semi_diff % 12) + 12) % 12;

    Interval::from_semitones(normalized, simple_degree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::pitch::Accidental;

    #[test]
    fn test_interval_semitones() {
        assert_eq!(Interval::unison().semitones(), 0);
        assert_eq!(Interval::minor_second().semitones(), 1);
        assert_eq!(Interval::major_second().semitones(), 2);
        assert_eq!(Interval::minor_third().semitones(), 3);
        assert_eq!(Interval::major_third().semitones(), 4);
        assert_eq!(Interval::perfect_fourth().semitones(), 5);
        assert_eq!(Interval::tritone().semitones(), 6);
        assert_eq!(Interval::perfect_fifth().semitones(), 7);
        assert_eq!(Interval::octave().semitones(), 12);
    }

    #[test]
    fn test_interval_from_semitones() {
        assert_eq!(Interval::from_semitones(0, 1), Interval::unison());
        assert_eq!(Interval::from_semitones(2, 2), Interval::major_second());
        assert_eq!(Interval::from_semitones(1, 2), Interval::minor_second());
        assert_eq!(Interval::from_semitones(4, 3), Interval::major_third());
        assert_eq!(Interval::from_semitones(3, 3), Interval::minor_third());
        assert_eq!(Interval::from_semitones(5, 4), Interval::perfect_fourth());
        assert_eq!(Interval::from_semitones(7, 5), Interval::perfect_fifth());
        assert_eq!(Interval::from_semitones(12, 8), Interval::octave());
    }

    #[test]
    fn test_interval_between_class() {
        let c = PitchClass::new('C', Accidental::Natural);
        let e = PitchClass::new('E', Accidental::Natural);
        let g = PitchClass::new('G', Accidental::Natural);
        let f_sharp = PitchClass::new('F', Accidental::Sharp);

        assert_eq!(interval_between(c, e), Interval::major_third());
        assert_eq!(interval_between(c, g), Interval::perfect_fifth());
        assert_eq!(interval_between(c, f_sharp), Interval::tritone());
    }

    #[test]
    fn test_interval_display() {
        assert_eq!(Interval::major_third().display(), "M3");
        assert_eq!(Interval::perfect_fifth().display(), "P5");
        assert_eq!(Interval::minor_seventh().display(), "m7");
    }

    #[test]
    fn test_interval_between_inverse() {
        let c = PitchClass::new('C', Accidental::Natural);
        let g = PitchClass::new('G', Accidental::Natural);
        // C→G = P5, G→C = P4
        assert_eq!(interval_between(c, g), Interval::perfect_fifth());
        assert_eq!(interval_between(g, c), Interval::perfect_fourth());
    }
}
