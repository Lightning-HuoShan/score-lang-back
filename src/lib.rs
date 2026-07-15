//! ScoreAnalysis - ScoreLang 乐谱解析与 MIDI 处理库
//!
//! 提供以下核心功能：
//! - ScoreLang 语法解析
//! - 音乐理论计算（音阶、和弦、调性）
//! - MIDI 文件生成
//! - 实时 MIDI 播放
//!
//! # 快速开始
//!
//! ```rust
//! use score_analysis::parse_score;
//! use score_analysis::export_midi;
//!
//! let score = parse_score(r#"@tempo(120) C4f4"#).unwrap();
//! export_midi(&score, std::path::Path::new("output.mid")).unwrap();
//! ```

pub mod analysis;
pub mod player;

pub use analysis::kind;
pub use analysis::parser;

pub use player::midi_writer;
pub use player::score_to_midi;
pub use player::live_player;

pub use analysis::parser::parser::parse_score;

pub use player::export_midi;
pub use player::export_midi_with_ppq;
pub use player::play_score;
pub use player::play_score_with_port;
pub use player::play_score_async;
pub use player::list_output_ports;
pub use player::score_to_midi::score_to_midi_with_ppq;
