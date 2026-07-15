#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locality {
    Global,
    Local,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConfigMarker {
    locality: Locality,
}

impl ConfigMarker {
    pub fn global() -> Self {
        Self { locality: Locality::Global }
    }

    pub fn local() -> Self {
        Self { locality: Locality::Local }
    }

    pub fn is_local(&self) -> bool {
        matches!(self.locality, Locality::Local)
    }

    pub fn is_global(&self) -> bool {
        matches!(self.locality, Locality::Global)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tempo {
    bpm: u16,
    marker: ConfigMarker,
}

impl Tempo {
    pub fn global(bpm: u16) -> Self {
        Self {
            bpm: bpm.clamp(1, 999),
            marker: ConfigMarker::global(),
        }
    }

    pub fn local(bpm: u16) -> Self {
        Self {
            bpm: bpm.clamp(1, 999),
            marker: ConfigMarker::local(),
        }
    }

    pub fn bpm(&self) -> u16 {
        self.bpm
    }

    pub fn set_bpm(&mut self, new_bpm: u16) {
        self.bpm = new_bpm.clamp(1, 999);
    }

    pub fn is_local(&self) -> bool {
        self.marker.is_local()
    }

    pub fn is_global(&self) -> bool {
        self.marker.is_global()
    }

    pub fn microseconds_per_beat(&self) -> u32 {
        60_000_000 / self.bpm.max(1) as u32
    }

    pub fn display(&self) -> String {
        format!("tempo({})", self.bpm)
    }
}
