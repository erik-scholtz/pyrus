use std::collections::HashMap;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Single-char symbols
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftBracket, RightBracket,
    Comma, Dot, Semicolon, Colon,
    Plus, Minus, Star, Slash, Percent,
    Equals, Dollarsign,

    // Literals
    Identifier,
    Int,
    Float,
    StringLiteral,

    // Keywords
    Template,
    Document,
    Style,
    Let,
    Const,
    If,
    Else,
    For,
    While,
    Return,

    // End
    Eof,
}

fn make_keyword_lookup_table() -> HashMap<&'static str, TokenKind> {
    use TokenKind::*;

    HashMap::from([
        ("template", Template),
        ("document", Document),
        ("style", Style),
        ("let", Let),
        ("const", Const),
        ("if", If),
        ("else", Else),
        ("for", For),
        ("while", While),
        ("return", Return),
    ])
}

static SYMBOL_LOOKUP_TABLE: [Option<TokenKind>; 256] = {
    let mut t = [None; 256];

    use TokenKind::*;

    t[b'(' as usize] = Some(LeftParen);
    t[b')' as usize] = Some(RightParen);
    t[b'{' as usize] = Some(LeftBrace);
    t[b'}' as usize] = Some(RightBrace);
    t[b'[' as usize] = Some(LeftBracket);
    t[b']' as usize] = Some(RightBracket);
    t[b',' as usize] = Some(Comma);
    t[b'.' as usize] = Some(Dot);
    t[b';' as usize] = Some(Semicolon);
    t[b':' as usize] = Some(Colon);
    t[b'+' as usize] = Some(Plus);
    t[b'-' as usize] = Some(Minus);
    t[b'*' as usize] = Some(Star);
    t[b'/' as usize] = Some(Slash);
    t[b'%' as usize] = Some(Percent);
    t[b'=' as usize] = Some(Equals);
    t[b'$' as usize] = Some(Dollarsign);

    t
};

////////////////
//// tokens ////
////////////////

#[derive(Debug)]
pub struct TokenStream {
    pub kinds: Vec<TokenKind>,
    pub ranges: Vec<std::ops::Range<usize>>,
    pub lines: Vec<u32>,
    pub cols: Vec<u32>,
    pub source: String,
}

impl TokenStream {
    pub fn new(source: String) -> Self {
        Self {
            kinds: Vec::new(),
            ranges: Vec::new(),
            lines: Vec::new(),
            cols: Vec::new(),
            source,
        }
    }

    #[inline]
    fn push(&mut self, kind: TokenKind, start: usize, end: usize, line: u32, col: u32) {
        self.kinds.push(kind);
        self.ranges.push(start..end);
        self.lines.push(line);
        self.cols.push(col);
    }
}

///////////////
//// lexer ////
///////////////

#[inline]
fn is_ident_start(c: u8) -> bool {
    (c >= b'a' && c <= b'z') ||
    (c >= b'A' && c <= b'Z') ||
     c == b'_'
}

#[inline]
fn is_ident_continue(c: u8) -> bool {
    is_ident_start(c) || (c >= b'0' && c <= b'9')
}


pub fn lex(source: &str) -> TokenStream {
    let mut out = TokenStream::new(source.to_string());
    let bytes = source.as_bytes();
    let len = bytes.len();

    let mut i = 0;
    let mut line = 1;
    let mut col = 1;

    while i < len {
        let c = bytes[i];

        // --- Whitespace ---
        if c.is_ascii_whitespace() {
            if c == b'\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
            i += 1;
            continue;
        }

        let start = i;

        // --- Identifiers ---
        if is_ident_start(c) {
            i += 1;
            while i < len && is_ident_continue(bytes[i]) {
                i += 1;
            }
            out.push(TokenKind::Identifier, start, i, line, col);
            col += (i - start) as u32;
            continue;
        }

        // --- Numbers ---
        if c.is_ascii_digit() {
            let mut is_float = false;
            i += 1;
            while i < len && bytes[i].is_ascii_digit() { i += 1; }

            if i < len && bytes[i] == b'.' {
                is_float = true;
                i += 1;
                while i < len && bytes[i].is_ascii_digit() { i += 1; }
            }

            let kind = if is_float { TokenKind::Float } else { TokenKind::Int };
            out.push(kind, start, i, line, col);
            col += (i - start) as u32;
            continue;
        }

        // --- String literals ---
        if c == b'"' {
            i += 1; // skip opening quote
            while i < len && bytes[i] != b'"' {
                if bytes[i] == b'\\' && i + 1 < len { i += 1; }
                i += 1;
            }
            i += 1; // skip closing quote
            out.push(TokenKind::StringLiteral, start, i, line, col);
            col += (i - start) as u32;
            continue;
        }

        // --- Comments ---
        if c == b'/' && i + 1 < len {
            if bytes[i + 1] == b'/' {
                i += 2;
                while i < len && bytes[i] != b'\n' { i += 1; }
                continue;
            } else if bytes[i + 1] == b'*' {
                i += 2;
                while i + 1 < len && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                    if bytes[i] == b'\n' { line += 1; col = 1; }
                    i += 1;
                }
                i += 2; // skip closing */
                continue;
            }
        }

        // --- Single-character tokens ---
        if let Some(kind) = SYMBOL_LOOKUP_TABLE[c as usize] {
            out.push(kind, i, i + 1, line, col);
            i += 1;
            col += 1;
            continue;
        }

        // --- Unknown character ---
        eprintln!("Unknown character: {:?}", c as char);
        i += 1;
        col += 1;
    }

    // --- EOF ---
    out.push(TokenKind::Eof, len, len, line, col);
    out
}

