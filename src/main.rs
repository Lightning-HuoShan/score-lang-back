use analysis::parser::lexer::Lexer;
use analysis::parser::parser::parse_score;
use player::score_to_midi;

fn main() {
  let src = r#"
@key(C major)
@tempo(120)
@time(4/4)
@title("Test Score")
track "PianoTrack" inst(piano) {
  section "Intro" repeat(2) {
      C4f4 D4f4 E4f4 F4f4 |
      G4f4 A4f4 B4f4 C5f4 |
  }
  section "Main" repeat(50) {
      [Cmaj7/E3]f4
  }
}
"#;

  println!("=== 词法分析 ===");
  let test_src = "C#4f4";
  let mut lex = Lexer::new(test_src);
  loop {
    let tok = lex.next_token();
    println!("  {:?} @ {:?}", tok.token, tok.span);
    if matches!(tok.token, analysis::parser::token::Token::Eof) {
      break;
    }
  }

  println!("\n=== 语法分析 ===");

  match parse_score(src) {
    Ok(score) => {
      println!("解析成功!");
      println!("标题: {:?}", score.title);
      println!("全局调: {:?}", score.global_key.as_ref().map(|k| k.display()));
      println!("全局速度: {:?}", score.global_tempo.as_ref().map(|t| t.bpm()));
      println!("全局拍号: {:?}", score.global_time.as_ref().map(|t| t.display()));
      println!("声部数: {}", score.tracks.len());
      for track in &score.tracks {
        println!("  声部 {}: {}", track.track_id, track.name);
        println!("    乐器: {:?}", track.instrument.as_ref().map(|i| i.display()));
        println!("    段落数: {}", track.sections.len());
        for section in &track.sections {
          println!("      段落: {} (重复: {:?})", section.name, section.repeat_times);
          println!("      小节数: {}", section.measures.len());
          for (i, measure) in section.measures.iter().enumerate() {
            println!("        小节 {}: {} 个事件", i, measure.events.len());
          }
        }
      }
      let notes = score.all_notes();
      println!("\n所有音符数: {}", notes.len());
      let chords = score.all_chords();
      println!("所有和弦数: {}", chords.len());
      for chord in &chords {
        println!("  和弦 {}: MIDI音 = {:?}", chord.display(), chord.midi_notes());
      }

      println!("\n=== MIDI 导出 ===");
      let midi = score_to_midi(&score);
      let bytes = midi.to_bytes();
      println!("MIDI 文件大小: {} 字节", bytes.len());
      println!("格式: Format {}", midi.format);
      println!("PPQ: {}", midi.division);
      println!("轨道数: {}", midi.track_count());

      let path = std::path::Path::new("output.mid");
      match player::export_midi(&score, path) {
        Ok(()) => println!("已导出: {:?}", path),
        Err(e) => println!("导出失败: {}", e),
      }

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
            Err(e) => println!("播放失败: {}", e),
          }
        }
        Err(e) => println!("无法列举端口: {}", e),
      }
    }
    Err(errors) => {
      println!("解析失败，共 {} 个错误:", errors.len());
      for e in &errors {
        println!("  - {}", e);
      }
    }
  }
}
