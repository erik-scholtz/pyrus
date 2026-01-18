use crate::lexer::TokenKind;
use crate::parser::parser::Parser;

impl Parser {
    pub fn current_token_kind(&self) -> TokenKind {
        self.toks.kinds[self.idx]
    }

    pub fn current_token_line(&self) -> u32 {
        self.toks.lines[self.idx]
    }

    pub fn current_token_col(&self) -> u32 {
        self.toks.cols[self.idx]
    }

    pub fn current_text(&self) -> String {
        let range = &self.toks.ranges[self.idx];
        self.toks.source[range.start..range.end].to_string()
    }

    pub fn advance(&mut self) -> TokenKind {
        if self.idx < self.toks.kinds.len() {
            self.idx += 1;
        }
        self.toks.kinds[self.idx - 1]
    }

    pub fn expect(&mut self, kind: TokenKind) -> TokenKind {
        if self.current_token_kind() == kind {
            return self.advance().clone();
        }
        panic!(
            "Parse error: expected {:?} but found {:?} at {}:{}",
            kind,
            self.current_token_kind(),
            self.current_token_line(),
            self.current_token_col()
        );
    }

    pub fn match_kind(&mut self, kind: TokenKind) -> bool {
        if self.current_token_kind() == kind {
            self.advance();
            return true;
        }
        false
    }

    /// If a block `{ ... }` follows, skip it including nested braces.
    pub fn skip_optional_block(&mut self) {
        // skip optional whitespace-free tokens; if next is LeftBrace, skip until matching RightBrace
        if self.idx < self.toks.kinds.len() && self.current_token_kind() == TokenKind::LeftBrace {
            let mut depth: i32 = 0;
            while self.idx < self.toks.kinds.len() {
                match self.current_token_kind() {
                    TokenKind::LeftBrace => {
                        depth += 1;
                    }
                    TokenKind::RightBrace => {
                        depth -= 1;
                        if depth <= 0 {
                            self.advance();
                            break;
                        }
                    }
                    TokenKind::Eof => break,
                    _ => {}
                }
                self.advance();
            }
        }
    }
}
