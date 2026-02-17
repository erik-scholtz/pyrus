use crate::parser::parser::Parser;

use crate::ast::{KeyValue, Selector, StyleRule};
use crate::lexer::TokenKind;

impl Parser {
    pub fn parse_style_block(&mut self) -> Vec<StyleRule> {
        let mut rules: Vec<StyleRule> = Vec::new();
        while self.idx < self.toks.kinds.len() {
            match self.current_token_kind() {
                TokenKind::RightBrace => {
                    self.advance(); // exit block
                    break;
                }
                TokenKind::Eof => break,
                _ => {
                    let statement = self.parse_style_rule();
                    rules.push(statement);
                }
            }
        }
        rules
    }
    pub fn parse_style_rule(&mut self) -> StyleRule {
        let selectors = self.parse_selector_list();
        let declarations = self.parse_style_declarations();
        StyleRule::new(selectors, declarations)
    }

    pub fn parse_selector_list(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        while self.idx < self.toks.kinds.len() {
            match self.current_token_kind() {
                TokenKind::Comma => {
                    self.advance(); // skip comma
                }
                TokenKind::LeftBrace => {
                    self.advance(); // exit selector list
                    break;
                }
                TokenKind::Eof => break,
                _ => {
                    let selector = match self.current_text().as_str() {
                        "." => {
                            self.advance();
                            Selector::Class(self.current_text().to_string())
                        }
                        "#" => {
                            self.advance();
                            Selector::Id(self.current_text().to_string())
                        }
                        _ => {
                            // TODO: have a check to make sure the type is valid CSS type
                            Selector::Type(self.current_text().to_string())
                        }
                    };
                    selectors.push(selector);
                    self.advance();
                }
            }
        }
        selectors
    }

    pub fn parse_style_declarations(&mut self) -> Vec<KeyValue> {
        let mut declarations = Vec::new();
        while self.idx < self.toks.kinds.len() {
            match self.current_token_kind() {
                TokenKind::Semicolon => {
                    self.advance(); // skip semicolon
                }
                TokenKind::RightBrace => {
                    self.advance(); // exit declaration block
                    break;
                }
                TokenKind::Eof => break,
                _ => {
                    let mut property: String = String::new();
                    while self.current_token_kind() != TokenKind::Equals {
                        property.push_str(&self.current_text().to_string());
                        self.advance();
                    }
                    self.advance(); // skip equals
                    let value = self.parse_expression();
                    declarations.push({
                        KeyValue {
                            key: property,
                            value: value,
                        }
                    });
                }
            }
        }
        declarations
    }
}
