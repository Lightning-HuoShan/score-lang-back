// ===================== 时值载体 =====================
/// 时值：语法 duration = (F/Dot) Number
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duration {
    /// 基础时值分母：f4 = 四分音符(4)，f2=二分音符(2)
    pub base: u32,
    /// 是否附点 . ，附点时值延长一半
    pub dotted: bool,
}

impl Duration {
    pub fn new(base: u32, dotted: bool) -> Self {
        Self { base, dotted }
    }

    /// 计算相对时值（以全音符=1.0为单位）
    pub fn value(&self) -> f32 {
        let unit = 1.0 / self.base.max(1) as f32;
        if self.dotted { unit * 1.5 } else { unit }
    }

    /// 以四分音符为单位的时值（f4 = 1.0）
    pub fn quarter_notes(&self) -> f32 {
        self.value() * 4.0
    }

    /// 计算该时值对应的微秒数
    /// bpm: 每分钟四分音符数
    pub fn microseconds(&self, bpm: u16) -> u64 {
        let bpm = bpm.max(1) as f64;
        let us_per_beat = 60_000_000.0 / bpm;
        let beats = self.quarter_notes() as f64;
        (us_per_beat * beats) as u64
    }

    /// 输出文本 f4 / .2
    pub fn display(&self) -> String {
        let prefix = if self.dotted { "." } else { "f" };
        format!("{}{}", prefix, self.base)
    }
}