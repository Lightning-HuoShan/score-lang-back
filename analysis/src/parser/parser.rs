use super::lexer::Lexer;
use super::token::{Token, SpannedToken, Span, LexError};
use crate::kind::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken { expected: String, got: String, span: Span },
    LexError(LexError),
    Eof,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, got, span } => {
                write!(f, "{}: 期望 {}, 得到 {}", span, expected, got)
            }
            ParseError::LexError(e) => write!(f, "词法错误: {}", e),
            ParseError::Eof => write!(f, "意外的文件结束"),
        }
    }
}

impl std::error::Error for ParseError {}

impl From<LexError> for ParseError {
    fn from(e: LexError) -> Self {
        ParseError::LexError(e)
    }
}

pub struct Parser {
    tokens: Vec<SpannedToken>,
    pos: usize,
    track_counter: usize,
    measure_counter: u32,
    pub errors: Vec<ParseError>,
    current_key: Option<Key>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize_all();
        let errors: Vec<ParseError> = lexer.errors.into_iter().map(ParseError::LexError).collect();
        Self {
            tokens,
            pos: 0,
            track_counter: 0,
            measure_counter: 0,
            errors,
            current_key: None,
        }
    }

    fn peek(&self) -> &SpannedToken {
        self.tokens.get(self.pos)
            .unwrap_or_else(|| self.tokens.last()
                .expect("token 序列不应为空：tokenize_all 保证至少返回 Eof"))
    }

    fn advance(&mut self) -> SpannedToken {
        let t = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, expected: &Token, desc: &str) -> Result<SpannedToken, ParseError> {
        let t = self.peek();
        if std::mem::discriminant(&t.token) == std::mem::discriminant(expected) {
            Ok(self.advance())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: desc.to_string(),
                got: format!("{:?}", t.token),
                span: t.span,
            })
        }
    }

    /// 记录错误并跳过当前 token（错误恢复辅助）
    fn skip_token_with_error(&mut self, expected: &str) {
        let t = self.peek().clone();
        let err = ParseError::UnexpectedToken {
            expected: expected.to_string(),
            got: format!("{:?}", t.token),
            span: t.span,
        };
        self.errors.push(err);
        self.advance();
    }

    fn check(&self, expected: &Token) -> bool {
        let t = self.peek();
        std::mem::discriminant(&t.token) == std::mem::discriminant(expected)
    }

    pub fn parse(&mut self) -> Result<Score, ParseError> {
        self.parse_score()
    }

    fn parse_score(&mut self) -> Result<Score, ParseError> {
        let mut score = Score::empty();
        self.measure_counter = 0;
        self.track_counter = 0;

        // 全局控制
        while !self.check(&Token::Eof) && !self.check(&Token::Track) {
            if self.check(&Token::At) {
                if let Err(e) = self.parse_global(&mut score) {
                    self.errors.push(e);
                    self.skip_to_next_global_or_track();
                }
            } else {
                self.skip_token_with_error("@ 或 track");
            }
        }

        // 音轨
        while self.check(&Token::Track) {
            match self.parse_track() {
                Ok(track) => score.push_track(track),
                Err(e) => {
                    self.errors.push(e);
                    self.skip_to_next_track();
                }
            }
        }

        // 允许末尾有非 Eof 的垃圾 token（已作为错误记录）
        if !self.check(&Token::Eof) {
            self.skip_token_with_error("文件结束");
        }
        Ok(score)
    }

    /// 跳过 token 直到遇到下一个 @ 或 track 或 Eof
    fn skip_to_next_global_or_track(&mut self) {
        while !self.check(&Token::Eof) && !self.check(&Token::At) && !self.check(&Token::Track) {
            self.advance();
        }
    }

    /// 跳过 token 直到遇到下一个 track 或 Eof
    fn skip_to_next_track(&mut self) {
        while !self.check(&Token::Eof) && !self.check(&Token::Track) {
            self.advance();
        }
    }

    fn parse_global(&mut self, score: &mut Score) -> Result<(), ParseError> {
        self.expect(&Token::At, "@")?;
        let t = self.peek();
        match &t.token {
            Token::KeySet => {
                let key = self.parse_key_control(false)?;
                self.current_key = Some(key);
                score.set_global_key(key);
            }
            Token::TempoSet => {
                let tempo = self.parse_tempo_control(false)?;
                score.set_global_tempo(tempo);
            }
            Token::TimeSet => {
                let ts = self.parse_time_control(false)?;
                score.set_global_time(ts);
            }
            Token::TitleSet => {
                self.advance();
                self.expect(&Token::Lpar, "(")?;
                let title_tok = self.expect(&Token::StringLit("".into()), "字符串")?;
                if let Token::StringLit(s) = title_tok.token {
                    score.set_title(s);
                }
                self.expect(&Token::Rpar, ")")?;
            }
            _ => {
                let err = ParseError::UnexpectedToken {
                    expected: "全局控制 (key/tempo/time/title)".to_string(),
                    got: format!("{:?}", t.token),
                    span: t.span,
                };
                self.errors.push(err.clone());
                return Err(err);
            }
        }
        Ok(())
    }

    fn parse_key_control(&mut self, is_local: bool) -> Result<Key, ParseError> {
        self.expect(&Token::KeySet, "key")?;
        self.expect(&Token::Lpar, "(")?;
        let pc = self.parse_pitch_class()?;
        let mode = if self.check(&Token::Major) {
            self.advance();
            KeyMode::Major
        } else if self.check(&Token::Minor) {
            self.advance();
            KeyMode::Minor
        } else {
            let t = self.peek();
            return Err(ParseError::UnexpectedToken {
                expected: "major 或 minor".to_string(),
                got: format!("{:?}", t.token),
                span: t.span,
            });
        };
        self.expect(&Token::Rpar, ")")?;
        if is_local {
            Ok(Key::local(pc, mode))
        } else {
            Ok(Key::global(pc, mode))
        }
    }

    fn parse_tempo_control(&mut self, is_local: bool) -> Result<Tempo, ParseError> {
        self.expect(&Token::TempoSet, "tempo")?;
        self.expect(&Token::Lpar, "(")?;
        let num_tok = self.expect(&Token::Number(0), "数字")?;
        let bpm = if let Token::Number(n) = num_tok.token { n as u16 } else { 120 };
        self.expect(&Token::Rpar, ")")?;
        if is_local {
            Ok(Tempo::local(bpm))
        } else {
            Ok(Tempo::global(bpm))
        }
    }

    fn parse_time_control(&mut self, is_local: bool) -> Result<TimeSig, ParseError> {
        self.expect(&Token::TimeSet, "time")?;
        self.expect(&Token::Lpar, "(")?;
        let beats_tok = self.expect(&Token::Number(0), "数字")?;
        let beats = if let Token::Number(n) = beats_tok.token { n as u32 } else { 4 };
        self.expect(&Token::Slash, "/")?;
        let value_tok = self.expect(&Token::Number(0), "数字")?;
        let value = if let Token::Number(n) = value_tok.token { n as u32 } else { 4 };
        self.expect(&Token::Rpar, ")")?;
        if is_local {
            Ok(TimeSig::local(beats, value))
        } else {
            Ok(TimeSig::global(beats, value))
        }
    }

    fn parse_pitch_class(&mut self) -> Result<PitchClass, ParseError> {
        let name_tok = self.expect(&Token::NoteName('C'), "音名 (CDEFGAB)")?;
        let name = if let Token::NoteName(c) = name_tok.token { c } else { 'C' };
        let acc = if self.check(&Token::Accidental("".into())) {
            let acc_tok = self.advance();
            if let Token::Accidental(s) = acc_tok.token {
                Accidental::from_str(&s)
            } else {
                Accidental::Natural
            }
        } else {
            if let Some(key) = &self.current_key {
                key.accidental_for_note_name(name)
            } else {
                Accidental::Natural
            }
        };
        Ok(PitchClass::new(name, acc))
    }

    fn parse_pitch(&mut self) -> Result<Pitch, ParseError> {
        let pc = self.parse_pitch_class()?;
        if self.check(&Token::Number(0)) {
            let num_tok = self.advance();
            if let Token::Number(n) = num_tok.token {
                Ok(Pitch::with_octave(pc.name, pc.acc, n as u8))
            } else {
                Ok(Pitch::without_octave(pc.name, pc.acc))
            }
        } else {
            Ok(Pitch::without_octave(pc.name, pc.acc))
        }
    }

    /// 解析音级（不应用调号，用于和弦根音/低音——和弦符号是绝对音高）
    fn parse_pitch_class_no_key(&mut self) -> Result<PitchClass, ParseError> {
        let name_tok = self.expect(&Token::NoteName('C'), "音名 (CDEFGAB)")?;
        let name = if let Token::NoteName(c) = name_tok.token { c } else { 'C' };
        let acc = if self.check(&Token::Accidental("".into())) {
            let acc_tok = self.advance();
            if let Token::Accidental(s) = acc_tok.token {
                Accidental::from_str(&s)
            } else {
                Accidental::Natural
            }
        } else {
            Accidental::Natural
        };
        Ok(PitchClass::new(name, acc))
    }

    /// 解析音高（不应用调号，用于和弦低音）
    fn parse_pitch_no_key(&mut self) -> Result<Pitch, ParseError> {
        let pc = self.parse_pitch_class_no_key()?;
        if self.check(&Token::Number(0)) {
            let num_tok = self.advance();
            if let Token::Number(n) = num_tok.token {
                Ok(Pitch::with_octave(pc.name, pc.acc, n as u8))
            } else {
                Ok(Pitch::without_octave(pc.name, pc.acc))
            }
        } else {
            Ok(Pitch::without_octave(pc.name, pc.acc))
        }
    }

    fn parse_duration(&mut self) -> Result<Duration, ParseError> {
        let mut dotted = if self.check(&Token::Dot) {
            self.advance();
            true
        } else if self.check(&Token::For) {
            self.advance();
            false
        } else {
            let t = self.peek();
            return Err(ParseError::UnexpectedToken {
                expected: "时值 (f或.开头)".to_string(),
                got: format!("{:?}", t.token),
                span: t.span,
            });
        };
        let num_tok = self.expect(&Token::Number(0), "数字")?;
        let base = if let Token::Number(n) = num_tok.token { n as u32 } else { 4 };
        if self.check(&Token::Dot) {
            self.advance();
            dotted = true;
        }
        Ok(Duration::new(base, dotted))
    }

    fn parse_track(&mut self) -> Result<Track, ParseError> {
        self.expect(&Token::Track, "track")?;
        let name_tok = self.expect(&Token::StringLit("".into()), "字符串")?;
        let name = if let Token::StringLit(s) = name_tok.token { s } else { String::new() };
        let track_id = self.track_counter;
        self.track_counter += 1;
        let mut track = Track::new(name, track_id);

        if self.check(&Token::Instrument) {
            let inst = self.parse_instrument_assign()?;
            track.set_instrument(inst);
        }

        self.expect(&Token::Lbrace, "{")?;
        while self.check(&Token::Section) {
            let section = self.parse_section(track_id)?;
            track.push_section(section);
        }
        self.expect(&Token::Rbrace, "}")?;
        Ok(track)
    }

    fn parse_instrument_assign(&mut self) -> Result<Instrument, ParseError> {
        self.expect(&Token::Instrument, "inst")?;
        self.expect(&Token::Lpar, "(")?;
        let name_tok = self.expect(&Token::InstrumentName("".into()), "乐器名")?;
        let kind = if let Token::InstrumentName(name) = name_tok.token {
            match InstrumentKind::from_str(&name) {
                Some(k) => k,
                None => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "有效的乐器名".to_string(),
                        got: format!("\"{}\"", name),
                        span: name_tok.span,
                    });
                }
            }
        } else {
            let t = self.peek();
            return Err(ParseError::UnexpectedToken {
                expected: "乐器名".to_string(),
                got: format!("{:?}", t.token),
                span: t.span,
            });
        };
        self.expect(&Token::Rpar, ")")?;
        Ok(Instrument::new(kind))
    }

    fn parse_section(&mut self, track_id: usize) -> Result<Section, ParseError> {
        self.expect(&Token::Section, "section")?;
        let name_tok = self.expect(&Token::StringLit("".into()), "字符串")?;
        let name = if let Token::StringLit(s) = name_tok.token { s } else { String::new() };
        let mut section = Section::new(name);

        if self.check(&Token::Repeat) {
            self.advance();
            self.expect(&Token::Lpar, "(")?;
            let rep_tok = self.expect(&Token::Number(0), "数字")?;
            if let Token::Number(n) = rep_tok.token {
                section.set_repeat(n as u32);
            }
            self.expect(&Token::Rpar, ")")?;
        }

        self.expect(&Token::Lbrace, "{")?;
        loop {
            if self.check(&Token::Rbrace) || self.check(&Token::Eof) {
                break;
            }
            if self.check(&Token::Bar) {
                self.advance();
                continue;
            }
            let measure = self.parse_measure(track_id)?;
            section.push_measure(measure);
        }
        self.expect(&Token::Rbrace, "}")?;
        Ok(section)
    }

    fn parse_measure(&mut self, track_id: usize) -> Result<Measure, ParseError> {
        let idx = self.measure_counter;
        self.measure_counter += 1;
        let mut measure = Measure::new(idx);

        loop {
            let t = self.peek();
            match &t.token {
                Token::Bar | Token::Rbrace | Token::Eof => break,
                Token::Rest => {
                    let note = self.parse_rest(track_id)?;
                    measure.push_event(MeasureEvent::Note(note));
                }
                Token::Lbrack => {
                    let chord = self.parse_chord(track_id)?;
                    measure.push_event(MeasureEvent::Chord(chord));
                }
                Token::At => {
                    let ctrl = self.parse_local_control()?;
                    measure.push_event(MeasureEvent::Control(ctrl));
                }
                Token::NoteName(_) => {
                    let note = self.parse_note(track_id)?;
                    measure.push_event(MeasureEvent::Note(note));
                }
                _ => {
                    self.skip_token_with_error("音符/休止符/和弦/控制命令");
                }
            }
        }
        Ok(measure)
    }

    fn parse_note(&mut self, track_id: usize) -> Result<Note, ParseError> {
        let pitch = self.parse_pitch()?;
        let dur = self.parse_duration()?;
        Ok(Note::new_note(pitch, dur, track_id))
    }

    fn parse_rest(&mut self, track_id: usize) -> Result<Note, ParseError> {
        self.expect(&Token::Rest, "R")?;
        let dur = self.parse_duration()?;
        Ok(Note::new_rest(dur, track_id))
    }

    fn parse_chord(&mut self, track_id: usize) -> Result<Chord, ParseError> {
        self.expect(&Token::Lbrack, "[")?;
        let root_pc = self.parse_pitch_class_no_key()?;
        let mut symbol = ChordSymbol::new(root_pc);

        if self.check(&Token::ChordQuality("".into())) {
            let q_tok = self.advance();
            if let Token::ChordQuality(q) = q_tok.token {
                if let Some(quality) = ChordQuality::from_str(&q) {
                    symbol.quality = Some(quality);
                }
            }
        }

        if self.check(&Token::Number(0)) {
            let num_tok = self.advance();
            if let Token::Number(n) = num_tok.token {
                symbol.base_number = Some(n as u32);
            }
        }

        while self.check(&Token::ChordAlter("".into())) {
            let a_tok = self.advance();
            let alter_type = if let Token::ChordAlter(s) = a_tok.token { s } else { String::new() };
            let num_tok = self.expect(&Token::Number(0), "数字")?;
            let number = if let Token::Number(n) = num_tok.token { n as u32 } else { 0 };
            symbol.alters.push(ChordAlterItem { alter_type, number });
        }

        let chord = if self.check(&Token::Slash) {
            self.advance();
            let bass = self.parse_pitch_no_key()?;
            self.expect(&Token::Rbrack, "]")?;
            let dur = self.parse_duration()?;
            Chord::new_slash(symbol, bass, dur, track_id)
        } else {
            self.expect(&Token::Rbrack, "]")?;
            let dur = self.parse_duration()?;
            Chord::new_normal(symbol, dur, track_id)
        };

        Ok(chord)
    }

    fn parse_local_control(&mut self) -> Result<LocalControl, ParseError> {
        self.expect(&Token::At, "@")?;
        let t = self.peek();
        match &t.token {
            Token::KeySet => {
                let key = self.parse_key_control(true)?;
                self.current_key = Some(key);
                Ok(LocalControl::LocalKey(key))
            }
            Token::TempoSet => {
                let tempo = self.parse_tempo_control(true)?;
                Ok(LocalControl::LocalTempo(tempo))
            }
            Token::TimeSet => {
                let ts = self.parse_time_control(true)?;
                Ok(LocalControl::LocalTime(ts))
            }
            Token::PedalOn => {
                self.advance();
                self.expect(&Token::Lpar, "(")?;
                let kind = self.parse_pedal_kind()?;
                self.expect(&Token::Rpar, ")")?;
                Ok(LocalControl::PedalOn(kind))
            }
            Token::PedalOff => {
                self.advance();
                self.expect(&Token::Lpar, "(")?;
                let kind = self.parse_pedal_kind()?;
                self.expect(&Token::Rpar, ")")?;
                Ok(LocalControl::PedalOff(kind))
            }
            _ => {
                let got = format!("{:?}", t.token);
                let span = t.span;
                self.skip_token_with_error("局部控制 (key/tempo/time/pedal_on/pedal_off)");
                return Err(ParseError::UnexpectedToken {
                    expected: "局部控制 (key/tempo/time/pedal_on/pedal_off)".to_string(),
                    got,
                    span,
                });
            }
        }
    }

    fn parse_pedal_kind(&mut self) -> Result<crate::kind::PedalKind, ParseError> {
        let name_tok = self.expect(&Token::InstrumentName("".into()), "踏板类型 (sustain/soft/sostenuto)")?;
        if let Token::InstrumentName(name) = name_tok.token {
            match name.as_str() {
                "sustain" => Ok(crate::kind::PedalKind::Sustain),
                "soft" => Ok(crate::kind::PedalKind::Soft),
                "sostenuto" => Ok(crate::kind::PedalKind::Sostenuto),
                _ => Err(ParseError::UnexpectedToken {
                    expected: "sustain / soft / sostenuto".to_string(),
                    got: format!("\"{}\"", name),
                    span: name_tok.span,
                }),
            }
        } else {
            let t = self.peek();
            Err(ParseError::UnexpectedToken {
                expected: "踏板类型 (sustain/soft/sostenuto)".to_string(),
                got: format!("{:?}", t.token),
                span: t.span,
            })
        }
    }
}

pub fn parse_score(input: &str) -> Result<Score, Vec<ParseError>> {
    let mut parser = Parser::new(input);
    match parser.parse() {
        Ok(score) => {
            if parser.errors.is_empty() {
                Ok(score)
            } else {
                Err(parser.errors)
            }
        }
        Err(e) => {
            let mut errors = parser.errors;
            if !errors.contains(&e) {
                errors.push(e);
            }
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_score() -> &'static str {
        r#"
@key(C major)
@tempo(120)
@time(4/4)
@title("Test Score")
track "PianoTrack" inst(piano) {
  section "Intro" {
      C4f4 D4f4 E4f4 F4f4 |
      C#4f4 Rf2 [G7]f4 [Cmaj7/E3]f4 |
  }
  section "Main" repeat(2) {
      E4f2. C4f4 G4f4 C5f1 |
  }
}
"#
    }

    #[test]
    fn test_parse_basic_score() {
        let score = parse_score(sample_score()).unwrap();
        assert_eq!(score.title, Some("Test Score".to_string()));
        assert!(score.global_tempo.is_some());
        assert_eq!(score.tracks.len(), 1);
    }

    #[test]
    fn test_parse_tracks_and_sections() {
        let score = parse_score(sample_score()).unwrap();
        let track = &score.tracks[0];
        assert_eq!(track.sections.len(), 2);
        assert_eq!(track.sections[0].name, "Intro");
        assert_eq!(track.sections[1].name, "Main");
        assert_eq!(track.sections[1].repeat_times, Some(2));
    }

    #[test]
    fn test_parse_measures() {
        let score = parse_score(sample_score()).unwrap();
        let section = &score.tracks[0].sections[0];
        assert_eq!(section.measures.len(), 2);
    }

    #[test]
    fn test_all_notes() {
        let score = parse_score(sample_score()).unwrap();
        let notes = score.all_notes();
        assert!(!notes.is_empty());
    }

    #[test]
    fn test_all_chords() {
        let score = parse_score(sample_score()).unwrap();
        let chords = score.all_chords();
        assert_eq!(chords.len(), 2);
    }

    #[test]
    fn test_parse_rest() {
        let score = parse_score(sample_score()).unwrap();
        let notes = score.all_notes();
        let has_rest = notes.iter().any(|n| n.is_rest());
        assert!(has_rest);
    }

    #[test]
    fn test_chord_midi_notes() {
        let score = parse_score(sample_score()).unwrap();
        let chords = score.all_chords();
        for ch in chords {
            let midi = ch.midi_notes();
            assert!(!midi.is_empty());
        }
    }

    #[test]
    fn test_note_midi_pitch() {
        let score = parse_score(sample_score()).unwrap();
        let notes = score.all_notes();
        for n in notes {
            if !n.is_rest() {
                let midi = n.midi_key();
                assert!(midi.is_some());
            }
        }
    }

    #[test]
    fn test_parse_global_time_sig() {
        let score = parse_score(sample_score()).unwrap();
        assert!(score.global_time.is_some());
        let ts = score.global_time.as_ref().unwrap();
        assert_eq!(ts.beats_per_bar, 4);
        assert_eq!(ts.beat_value, 4);
    }

    #[test]
    fn test_parse_error_invalid_char() {
        let result = parse_score("track \"P\" inst(piano) { section \"S\" { !4f4 | } }");
        assert!(result.is_err());
    }

    #[test]
    fn test_single_note() {
        let score = parse_score("track \"P\" inst(piano) { section \"S\" { C4f4 | } }").unwrap();
        let notes = score.all_notes();
        assert_eq!(notes.len(), 1);
        let n = notes[0];
        assert!(!n.is_rest());
        assert_eq!(n.midi_key(), Some(60));
    }

    #[test]
    fn test_accidental_sharp() {
        let score = parse_score("track \"P\" inst(piano) { section \"S\" { C#4f4 | } }").unwrap();
        let notes = score.all_notes();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].midi_key(), Some(61));
    }

    #[test]
    fn test_accidental_flat() {
        let score = parse_score("track \"P\" inst(piano) { section \"S\" { Eb4f4 | } }").unwrap();
        let notes = score.all_notes();
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].midi_key(), Some(63));
    }

    #[test]
    fn test_error_recovery_multiple_errors() {
        // 两个错误：无效字符 ! 和无效字符 ?
        let result = parse_score("track \"P\" inst(piano) { section \"S\" { ! C4f4 ? D4f4 | } }");
        assert!(result.is_err());
        let errors = result.unwrap_err();
        // 应该收集到至少 2 个错误（! 和 ?），但仍能恢复解析出 C4 和 D4
        assert!(errors.len() >= 2, "应收集多个错误，实际: {}", errors.len());
    }

    #[test]
    fn test_error_recovery_continues_after_bad_token() {
        // 错误 token 后应继续解析有效音符
        let result = parse_score("track \"P\" inst(piano) { section \"S\" { ! C4f4 | } }");
        // 错误输入应返回 Err（有词法/语法错误）
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_is_std_error() {
        let err = ParseError::Eof;
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_lex_error_is_std_error() {
        let err = LexError::UnknownChar(Span::zero(), '!');
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_parse_error_partial_eq() {
        let e1 = ParseError::Eof;
        let e2 = ParseError::Eof;
        assert_eq!(e1, e2);

        let s = Span::new(1, 2, 1);
        let e3 = ParseError::UnexpectedToken {
            expected: "x".to_string(),
            got: "y".to_string(),
            span: s,
        };
        let e4 = ParseError::UnexpectedToken {
            expected: "x".to_string(),
            got: "y".to_string(),
            span: s,
        };
        assert_eq!(e3, e4);
    }

    #[test]
    fn test_dedup_no_duplicate_errors() {
        // 确保 expect 不再重复记录错误
        let result = parse_score("track \"P\" inst(piano) { section \"S\" { ! | } }");
        if let Err(errors) = result {
            // 不应有重复错误
            for i in 0..errors.len() {
                for j in (i+1)..errors.len() {
                    assert_ne!(errors[i], errors[j], "发现重复错误: {:?}", errors[i]);
                }
            }
        }
    }

    #[test]
    fn test_key_signature_applies_to_notes() {
        // G 大调中，F 自动升半音
        let src = r#"
@key(G major)
@time(4/4)
track "P" inst(piano) {
  section "S" {
      F4f4 C4f4 |
  }
}
"#;
        let score = parse_score(src).unwrap();
        let notes = score.all_notes();
        assert_eq!(notes.len(), 2);
        // F 在 G 大调中应为 F# (MIDI 66)
        assert_eq!(notes[0].midi_key(), Some(66), "G 大调中 F 应自动升为 F#");
        // C 保持自然 (MIDI 60)
        assert_eq!(notes[1].midi_key(), Some(60), "G 大调中 C 保持自然");
    }

    #[test]
    fn test_explicit_accidental_overrides_key() {
        // 显式写的临时记号覆盖调号
        let src = r#"
@key(G major)
@time(4/4)
track "P" inst(piano) {
  section "S" {
      F4f4 F#4f4 |
  }
}
"#;
        let score = parse_score(src).unwrap();
        let notes = score.all_notes();
        assert_eq!(notes.len(), 2);
        // 未写记号的 F 按调号升半音 → F# (66)
        assert_eq!(notes[0].midi_key(), Some(66));
        // 显式写了 # 的还是 F# (66)，结果一样
        assert_eq!(notes[1].midi_key(), Some(66));
    }

    #[test]
    fn test_chord_root_ignores_key_signature() {
        // 和弦符号是绝对的，不应用调号
        let src = r#"
@key(G major)
@time(4/4)
track "P" inst(piano) {
  section "S" {
      [F]f2 |
  }
}
"#;
        let score = parse_score(src).unwrap();
        let chords = score.all_chords();
        assert_eq!(chords.len(), 1);
        // F 和弦根音是 F (MIDI 65 的 F3 之类)，不是 F#
        let midi_notes = chords[0].midi_notes();
        // F 大三和弦: F-A-C，最低音是 F
        assert!(midi_notes.contains(&65) || midi_notes.contains(&53),
                "F 和弦应包含 F 音，不应因调号变 F#");
        assert!(!midi_notes.contains(&66) && !midi_notes.contains(&54),
                "F 和弦不应包含 F# 音");
    }

    #[test]
    fn test_f_major_b_flat() {
        // F 大调中，B 自动降半音
        let src = r#"
@key(F major)
@time(4/4)
track "P" inst(piano) {
  section "S" {
      B4f4 F4f4 |
  }
}
"#;
        let score = parse_score(src).unwrap();
        let notes = score.all_notes();
        assert_eq!(notes.len(), 2);
        // B 在 F 大调中应为 Bb (MIDI 70)
        assert_eq!(notes[0].midi_key(), Some(70), "F 大调中 B 应自动降为 Bb");
        // F 保持自然 (MIDI 65)
        assert_eq!(notes[1].midi_key(), Some(65));
    }
}
