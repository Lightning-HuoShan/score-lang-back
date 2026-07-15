//! ScoreAnalysis Player - MIDI 导出与实时播放模块
//!
//! 将 ScoreLang 解析后的 Score 转换为标准 MIDI 文件 (SMF)，
//! 或通过系统 MIDI 合成器实时播放

pub mod midi_writer;
pub mod score_to_midi;
pub mod live_player;

pub use midi_writer::MidiFile;
pub use score_to_midi::{score_to_midi, score_to_midi_with_ppq};
pub use live_player::{play_score, play_score_with_port, play_score_async, list_output_ports};

use std::fs;
use std::io::Write;
use std::path::Path;

pub fn export_midi(score: &crate::analysis::kind::score::Score, path: &Path) -> std::io::Result<()> {
    let midi = score_to_midi(score);
    let bytes = midi.to_bytes();
    let mut file = fs::File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

pub fn export_midi_with_ppq(
    score: &crate::analysis::kind::score::Score,
    path: &Path,
    ppq: u16,
) -> std::io::Result<()> {
    let midi = score_to_midi_with_ppq(score, ppq);
    let bytes = midi.to_bytes();
    let mut file = fs::File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}
