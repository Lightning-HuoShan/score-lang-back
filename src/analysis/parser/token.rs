#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
    pub pos: usize,
}

impl Span {
    pub fn new(line: usize, col: usize, pos: usize) -> Self {
        Self { line, col, pos }
    }

    pub fn zero() -> Self {
        Self { line: 1, col: 1, pos: 0 }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

impl SpannedToken {
    pub fn new(token: Token, span: Span) -> Self {
        Self { token, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    At,
    Lbrace, Rbrace,
    Lbrack, Rbrack,
    Lpar, Rpar,
    Bar, Comma, Slash, Dot,

    For, Rest,

    KeySet, TempoSet, TimeSet, TitleSet,
    Major, Minor,

    Track, Section, Repeat, Instrument,

    PedalOn, PedalOff,

    NoteName(char),

    Accidental(String),

    Number(u64),

    InstrumentName(String),

    ChordQuality(String),
    ChordAlter(String),

    StringLit(String),

    UnclosedString,
    UnknownChar(char),
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnclosedString(Span),
    UnknownChar(Span, char),
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnclosedString(span) => write!(f, "{}: 未闭合的字符串", span),
            LexError::UnknownChar(span, c) => write!(f, "{}: 未知字符 '{}'", span, c),
        }
    }
}

impl std::error::Error for LexError {}
