use super::token::{Token, Span, SpannedToken, LexError};

#[derive(Debug, Clone, Default)]
pub struct LexConfig {
    pub comment_start: char,
    pub enable_comment: bool,
    pub string_delimiter: char,
    pub keyword_case_sensitive: bool,
}

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    idx: usize,
    size: usize,
    line: usize,
    col: usize,
    pub config: LexConfig,
    pub errors: Vec<LexError>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self::new_with_config(input, LexConfig::default_score_lang())
    }

    pub fn new_with_config(input: &str, config: LexConfig) -> Self {
        let input: Vec<char> = input.chars().collect();
        let size = input.len();
        Self {
            input,
            idx: 0,
            size,
            line: 1,
            col: 1,
            config,
            errors: Vec::new(),
        }
    }

    pub fn tokenize_all(&mut self) -> Vec<SpannedToken> {
        let mut tokens = Vec::new();
        loop {
            let t = self.next_token();
            let is_eof = matches!(t.token, Token::Eof);
            tokens.push(t);
            if is_eof { break; }
        }
        tokens
    }

    fn current_span(&self) -> Span {
        Span::new(self.line, self.col, self.idx)
    }

    fn advance(&mut self) -> Option<char> {
        if self.idx < self.size {
            let ch = self.input[self.idx];
            self.idx += 1;
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        let pos = self.idx + offset - 1;
        if pos < self.size && pos >= self.idx {
            Some(self.input[pos])
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek(1) {
                Some(c) if c.is_ascii_whitespace() => {
                    self.advance();
                }
                _ => return,
            }
        }
    }

    fn skip_comment(&mut self) {
        loop {
            match self.peek(1) {
                Some('\n') | None => return,
                Some(_) => { self.advance(); }
            }
        }
    }

    fn skip_irrelevant(&mut self) -> bool {
        loop {
            self.skip_whitespace();
            if !self.config.enable_comment {
                return self.idx < self.size;
            }
            match self.peek(1) {
                Some(c) if c == self.config.comment_start => {
                    if self.is_hash_accidental() {
                        return true;
                    } else {
                        self.advance();
                        self.skip_comment();
                        continue;
                    }
                }
                Some(_) => return true,
                None => return false,
            }
        }
    }

    fn is_hash_accidental(&self) -> bool {
        if self.idx == 0 {
            return false;
        }
        let mut i = self.idx;
        while i > 0 && self.input[i - 1].is_ascii_whitespace() {
            i -= 1;
            if i > 0 && self.input[i - 1] == '\n' {
                return false;
            }
        }
        if i == 0 {
            return false;
        }
        let prev = self.input[i - 1];
        "ABCDEFGabcdefg".contains(prev)
    }

    fn read_ident(&mut self) -> String {
        let mut buf = String::new();
        buf.reserve(16);
        loop {
            match self.peek(1) {
                Some(c) if c.is_ascii_alphabetic() => {
                    self.advance();
                    buf.push(c);
                }
                _ => break,
            }
        }
        buf
    }

    fn read_number(&mut self) -> u64 {
        let mut num_str = String::new();
        loop {
            match self.peek(1) {
                Some(c) if c.is_ascii_digit() => {
                    self.advance();
                    num_str.push(c);
                }
                _ => break,
            }
        }
        num_str.parse().unwrap_or(0)
    }

    fn read_string_inner(&mut self) -> Option<String> {
        let mut buf = String::new();
        let end = self.config.string_delimiter;
        loop {
            match self.peek(1) {
                Some(c) if c == end => return Some(buf),
                None => return None,
                Some(ch) => {
                    self.advance();
                    buf.push(ch);
                }
            }
        }
    }

    fn ident_to_token(s: &str) -> Token {
        match s {
            "key" => Token::KeySet,
            "tempo" => Token::TempoSet,
            "time" => Token::TimeSet,
            "title" => Token::TitleSet,
            "major" => Token::Major,
            "minor" => Token::Minor,
            "track" => Token::Track,
            "section" => Token::Section,
            "repeat" => Token::Repeat,
            "inst" => Token::Instrument,
            "piano" => Token::Piano,
            "maj" | "min" | "m" | "dim" | "aug" | "sus" => Token::ChordQuality(s.to_string()),
            "add" | "flat" | "sharp" | "no" => Token::ChordAlter(s.to_string()),
            "f" => Token::For,
            "b" | "bb" | "x" => Token::Accidental(s.to_string()),
            _ => Token::UnknownChar(s.chars().next().unwrap_or('?')),
        }
    }

    pub fn next_token(&mut self) -> SpannedToken {
        if !self.skip_irrelevant() {
            return SpannedToken::new(Token::Eof, self.current_span());
        }

        let start_span = self.current_span();
        let ch = match self.peek(1) {
            Some(c) => c,
            None => return SpannedToken::new(Token::Eof, start_span),
        };

        let token = match ch {
            '@' => { self.advance(); Token::At }
            '{' => { self.advance(); Token::Lbrace }
            '}' => { self.advance(); Token::Rbrace }
            '[' => { self.advance(); Token::Lbrack }
            ']' => { self.advance(); Token::Rbrack }
            '(' => { self.advance(); Token::Lpar }
            ')' => { self.advance(); Token::Rpar }
            '|' => { self.advance(); Token::Bar }
            ',' => { self.advance(); Token::Comma }
            '/' => { self.advance(); Token::Slash }
            '.' => { self.advance(); Token::Dot }
            '#' => { self.advance(); Token::Accidental("#".to_string()) }
            '=' => { self.advance(); Token::Accidental("=".to_string()) }
            '"' => {
                self.advance();
                match self.read_string_inner() {
                    Some(text) => {
                        self.advance();
                        Token::StringLit(text)
                    }
                    None => {
                        self.errors.push(LexError::UnclosedString(start_span));
                        Token::UnclosedString
                    }
                }
            }
            '0'..='9' => Token::Number(self.read_number()),
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' => {
                self.advance();
                Token::NoteName(ch)
            }
            'R' => {
                self.advance();
                Token::Rest
            }
            'H'..='Q' | 'S'..='Z' => {
                self.advance();
                Token::UnknownChar(ch)
            }
            'a'..='z' => {
                let ident = self.read_ident();
                Self::ident_to_token(&ident)
            }
            c => {
                self.advance();
                self.errors.push(LexError::UnknownChar(start_span, c));
                Token::UnknownChar(c)
            }
        };

        SpannedToken::new(token, start_span)
    }
}

impl LexConfig {
    pub fn default_score_lang() -> Self {
        Self {
            comment_start: '#',
            enable_comment: false,
            string_delimiter: '"',
            keyword_case_sensitive: true,
        }
    }
}
