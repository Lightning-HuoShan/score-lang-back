pub const MIDI_CC_VOLUME: u8 = 7;
pub const MIDI_CC_PAN: u8 = 10;
pub const MIDI_CC_MIN: u8 = 0;
pub const MIDI_CC_MAX: u8 = 127;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstrumentKind {
    Piano,
}

impl InstrumentKind {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "piano" => Some(Self::Piano),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            InstrumentKind::Piano => "piano",
        }
    }

    pub fn midi_program(&self) -> u8 {
        match self {
            InstrumentKind::Piano => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instrument {
    pub kind: InstrumentKind,
    volume: u8,
    pan: u8,
}

impl Instrument {
    pub fn new(kind: InstrumentKind) -> Self {
        Self {
            kind,
            volume: 100,
            pan: 64,
        }
    }

    pub fn volume(&self) -> u8 {
        self.volume
    }

    pub fn pan(&self) -> u8 {
        self.pan
    }

    pub fn set_volume(&mut self, vol: u8) {
        self.volume = vol.clamp(MIDI_CC_MIN, MIDI_CC_MAX);
    }

    pub fn set_pan(&mut self, pan_val: u8) {
        self.pan = pan_val.clamp(MIDI_CC_MIN, MIDI_CC_MAX);
    }

    pub fn midi_program(&self) -> u8 {
        self.kind.midi_program()
    }

    pub fn midi_cc_volume(&self) -> (u8, u8) {
        (MIDI_CC_VOLUME, self.volume)
    }

    pub fn midi_cc_pan(&self) -> (u8, u8) {
        (MIDI_CC_PAN, self.pan)
    }

    pub fn display(&self) -> String {
        format!("inst({})", self.kind.as_str())
    }
}
