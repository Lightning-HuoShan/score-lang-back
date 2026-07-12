//! 标准 MIDI 文件 (SMF) 二进制写入器

/// MIDI 元事件类型
const META_EVENT: u8 = 0xFF;
const META_END_OF_TRACK: u8 = 0x2F;
const META_TEMPO: u8 = 0x51;
const META_TRACK_NAME: u8 = 0x03;

/// MIDI 通道事件类型
const NOTE_OFF: u8 = 0x80;
const NOTE_ON: u8 = 0x90;
const CONTROL_CHANGE: u8 = 0xB0;
const PROGRAM_CHANGE: u8 = 0xC0;

/// 一个带时间戳的 MIDI 事件
#[derive(Debug, Clone)]
struct TimedEvent {
    /// 相对于前一个事件的 delta ticks
    delta: u32,
    data: Vec<u8>,
}

/// 单个 MIDI 轨道
pub struct MidiTrack {
    events: Vec<TimedEvent>,
    name: Option<String>,
}

impl MidiTrack {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            name: None,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string());
    }

    /// 添加一个事件，delta 为距前一事件的 tick 数
    pub fn add_event(&mut self, delta: u32, data: Vec<u8>) {
        self.events.push(TimedEvent { delta, data });
    }

    /// 在轨道末尾添加固定时长的空白（仅推进时间）
    pub fn advance(&mut self, delta: u32) {
        if let Some(last) = self.events.last_mut() {
            last.delta += delta;
        } else {
            // 没有前序事件时，添加一个空 meta 事件作为占位
            self.events.push(TimedEvent {
                delta,
                data: vec![],
            });
        }
    }

    /// 序列化为 MTrk 字节流（含 MTrk 头和长度）
    fn serialize(&self) -> Vec<u8> {
        let mut body = Vec::new();

        // 轨道名
        if let Some(ref name) = self.name {
            let name_bytes = name.as_bytes();
            body.push(META_EVENT);
            body.push(META_TRACK_NAME);
            write_vlq(&mut body, name_bytes.len() as u32);
            body.extend_from_slice(name_bytes);
        }

        // 所有事件
        for event in &self.events {
            write_vlq(&mut body, event.delta);
            body.extend_from_slice(&event.data);
        }

        // End of Track
        write_vlq(&mut body, 0);
        body.push(META_EVENT);
        body.push(META_END_OF_TRACK);
        body.push(0);

        // MTrk 头
        let mut output = Vec::with_capacity(body.len() + 8);
        output.extend_from_slice(b"MTrk");
        output.extend_from_slice(&(body.len() as u32).to_be_bytes());
        output.extend_from_slice(&body);
        output
    }
}

/// 完整的 MIDI 文件
pub struct MidiFile {
    pub format: u16,
    pub division: u16, // ticks per quarter note (PPQ)
    tracks: Vec<MidiTrack>,
}

impl MidiFile {
    pub fn new(format: u16, ppq: u16) -> Self {
        Self {
            format,
            division: ppq,
            tracks: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track: MidiTrack) {
        self.tracks.push(track);
    }

    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// 序列化为完整的 SMF 字节流
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut output = Vec::new();

        // MThd 头
        output.extend_from_slice(b"MThd");
        output.extend_from_slice(&6u32.to_be_bytes()); // header length
        output.extend_from_slice(&self.format.to_be_bytes());
        output.extend_from_slice(&(self.tracks.len() as u16).to_be_bytes());
        output.extend_from_slice(&self.division.to_be_bytes());

        // 各轨道
        for track in &self.tracks {
            output.extend_from_slice(&track.serialize());
        }

        output
    }
}

// ===== 事件构造辅助函数 =====

/// Note On 事件
pub fn note_on(channel: u8, note: u8, velocity: u8) -> Vec<u8> {
    vec![NOTE_ON | (channel & 0x0F), note & 0x7F, velocity & 0x7F]
}

/// Note Off 事件
pub fn note_off(channel: u8, note: u8, velocity: u8) -> Vec<u8> {
    vec![NOTE_OFF | (channel & 0x0F), note & 0x7F, velocity & 0x7F]
}

/// Program Change 事件
pub fn program_change(channel: u8, program: u8) -> Vec<u8> {
    vec![PROGRAM_CHANGE | (channel & 0x0F), program & 0x7F]
}

/// Control Change 事件
pub fn control_change(channel: u8, controller: u8, value: u8) -> Vec<u8> {
    vec![
        CONTROL_CHANGE | (channel & 0x0F),
        controller & 0x7F,
        value & 0x7F,
    ]
}

/// Tempo 元事件 (microseconds per quarter note)
pub fn tempo_meta(us_per_quarter: u32) -> Vec<u8> {
    let bytes = us_per_quarter.to_be_bytes();
    vec![META_EVENT, META_TEMPO, 0x03, bytes[1], bytes[2], bytes[3]]
}

/// 写入变量长度量 (VLQ)
fn write_vlq(out: &mut Vec<u8>, mut value: u32) {
    if value == 0 {
        out.push(0);
        return;
    }
    let mut buffer = [0u8; 5];
    let mut idx = 0;
    buffer[idx] = (value & 0x7F) as u8;
    value >>= 7;
    while value > 0 {
        idx += 1;
        buffer[idx] = ((value & 0x7F) | 0x80) as u8;
        value >>= 7;
    }
    // 反向写入
    for i in (0..=idx).rev() {
        out.push(buffer[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vlq_encoding() {
        let mut buf = Vec::new();
        write_vlq(&mut buf, 0);
        assert_eq!(buf, vec![0]);

        let mut buf = Vec::new();
        write_vlq(&mut buf, 127);
        assert_eq!(buf, vec![127]);

        let mut buf = Vec::new();
        write_vlq(&mut buf, 128);
        assert_eq!(buf, vec![0x81, 0x00]);

        let mut buf = Vec::new();
        write_vlq(&mut buf, 480);
        assert_eq!(buf, vec![0x83, 0x60]);
    }

    #[test]
    fn test_note_on_event() {
        let ev = note_on(0, 60, 100);
        assert_eq!(ev, vec![0x90, 60, 100]);
    }

    #[test]
    fn test_tempo_meta() {
        let ev = tempo_meta(500000); // 120 BPM
        assert_eq!(ev, vec![0xFF, 0x51, 0x03, 0x07, 0xA1, 0x20]);
    }

    #[test]
    fn test_midi_file_header() {
        let file = MidiFile::new(1, 480);
        let bytes = file.to_bytes();
        // MThd
        assert_eq!(&bytes[0..4], b"MThd");
        // header length = 6
        assert_eq!(&bytes[4..8], &[0, 0, 0, 6]);
        // format = 1
        assert_eq!(&bytes[8..10], &[0, 1]);
        // track count = 0
        assert_eq!(&bytes[10..12], &[0, 0]);
        // division = 480
        assert_eq!(&bytes[12..14], &[0x01, 0xE0]);
    }

    #[test]
    fn test_empty_track_end_of_track() {
        let track = MidiTrack::new();
        let file = MidiFile::new(0, 480);
        let _file = file;
        let bytes = track.serialize();
        // MTrk header
        assert_eq!(&bytes[0..4], b"MTrk");
        // body should contain at least End of Track
        let body_len = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        assert!(body_len >= 3); // at least 3 bytes for EOT
    }
}
