use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

use analysis::parser::parser::parse_score;
use player::score_to_midi;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();

  let (src, source_name) = if args.len() > 1 {
    let path = Path::new(&args[1]);
    let content = fs::read_to_string(path)?;
    let name = path.file_name()
      .and_then(|n| n.to_str())
      .unwrap_or("unknown")
      .to_string();
    (content, name)
  } else {
    (DEFAULT_SCORE.to_string(), "(内置示例)".to_string())
  };

  println!("=== 乐谱来源: {} ===", source_name);

  println!("\n=== 语法分析 ===");

  let score = match parse_score(&src) {
    Ok(score) => {
      println!("解析成功!");
      println!("标题: {:?}", score.title);
      println!("全局调: {:?}", score.global_key.as_ref().map(|k| k.display()));
      println!("全局速度: {:?}", score.global_tempo.as_ref().map(|t| t.bpm()));
      println!("全局拍号: {:?}", score.global_time.as_ref().map(|t| t.display()));
      println!("声部数: {}", score.tracks.len());
      for track in &score.tracks {
        println!("  声部 {}: {} ({})", track.track_id, track.name, track.instrument.as_ref().map(|i| i.display()).unwrap_or_default());
        println!("    段落数: {}", track.sections.len());
      }
      let notes = score.all_notes();
      println!("\n所有音符数: {}", notes.len());
      let chords = score.all_chords();
      println!("所有和弦数: {}", chords.len());
      score
    }
    Err(errors) => {
      eprintln!("解析失败，共 {} 个错误:", errors.len());
      for e in &errors {
        eprintln!("  - {}", e);
      }
      std::process::exit(1);
    }
  };

  println!("\n=== MIDI 导出 ===");
  let midi = score_to_midi(&score);
  let bytes = midi.to_bytes();
  println!("MIDI 文件大小: {} 字节", bytes.len());
  println!("格式: Format {}", midi.format);
  println!("PPQ: {}", midi.division);
  println!("轨道数: {}", midi.track_count());

  let midi_path = if args.len() > 1 {
    let path = Path::new(&args[1]);
    path.with_extension("mid")
  } else {
    Path::new("output.mid").to_path_buf()
  };
  player::export_midi(&score, &midi_path)?;
  println!("已导出: {:?}", midi_path);

  println!("\n=== 实时播放 ===");
  match player::list_output_ports() {
    Ok(ports) => {
      println!("可用 MIDI 端口:");
      for (i, name) in ports.iter().enumerate() {
        println!("  [{}] {}", i, name);
      }
      print!("\n播放中... ");
      match player::play_score(&score) {
        Ok(()) => println!("播放完成!"),
        Err(e) => {
          eprintln!("播放失败: {}", e);
        }
      }
    }
    Err(e) => eprintln!("无法列举端口: {}", e),
  }

  Ok(())
}

const DEFAULT_SCORE: &str = r#"
@key(A minor)
@tempo(105)
@time(3/8)
@title("Für Elise - Beethoven")
track "RightHand" inst(bright_piano) {
  section "PartA1" {
      @pedal_on(sustain) E5f8 D#5f8 E5f8 | D#5f8 E5f8 B4f8 | D5f8 C5f8 A4.4 |
      Rf4 Rf8 | Rf4 Rf8 | @pedal_off(sustain) Rf4 Rf8 |
  }
  section "PartA2" {
      @pedal_on(sustain) E5f8 D#5f8 E5f8 | D#5f8 E5f8 B4f8 | D5f8 C5f8 A4.4 |
      Rf4 Rf8 | B4f8 C5f8 D5f8 | D5f8 E5f8 D5f8 |
      C5f8 D5f8 B4f8 | A4f8 Rf8 @pedal_off(sustain) Rf8 |
  }
  section "PartB" {
      @pedal_on(soft) E5f8 E5f8 E5f8 | E5f8 E5f8 G5f8 | C6f4 Rf8 |
      B5f4 Rf8 | A5f4 Rf8 | B5f4 C6f8 |
      D6f4 Rf8 | C6f4 G5f8 | E5f8 E5f8 E5f8 |
      E5f4 D5f8 | D6f4 C6f8 | C6.4 |
  }
  section "PartA3" {
      @pedal_off(soft) @pedal_on(sustain) E5f8 D#5f8 E5f8 | D#5f8 E5f8 B4f8 | D5f8 C5f8 A4.4 |
      Rf4 Rf8 | Rf4 Rf8 | Rf4 Rf8 |
  }
}
track "LeftHand" inst(bright_piano) {
  section "PartA1" {
      Rf4 Rf8 | Rf4 Rf8 | A2f8 E3f8 A3f8 |
      E2f8 E3f8 G#3f8 | A2f8 E3f8 A3f8 | E2f8 E3f8 G#3f8 |
  }
  section "PartA2" {
      Rf4 Rf8 | Rf4 Rf8 | A2f8 E3f8 A3f8 |
      E2f8 E3f8 G#3f8 | A2f8 E3f8 A3f8 | G2f8 G3f8 B3f8 |
      C3f8 G3f8 C4f8 | G2f8 G3f8 B3f8 |
  }
  section "PartB" {
      C3f8 G3f8 C4f8 | G2f8 G3f8 B3f8 | C3f8 G3f8 C4f8 |
      G2f8 G3f8 B3f8 | C3f8 G3f8 C4f8 | F2f8 C3f8 F3f8 |
      G2f8 G3f8 B3f8 | C3f8 G3f8 C4f8 | C3f8 G3f8 C4f8 |
      G2f8 G3f8 B3f8 | G2f8 G3f8 B3f8 | C3.4 |
  }
  section "PartA3" {
      Rf4 Rf8 | Rf4 Rf8 | A2f8 E3f8 A3f8 |
      E2f8 E3f8 G#3f8 | A2f8 E3f8 A3f8 | E2f8 E3f8 G#3f8 |
  }
}
"#;
