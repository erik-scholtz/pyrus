use core::panic;

use crate::ast::{Ast, DocumentBlock, Expression, StyleBlock, TemplateBlock};
use crate::lexer::{TokenKind, TokenStream};

pub fn parse(tokens: TokenStream) -> Ast {
    let p = Parser::new(tokens);
    p.parse()
}

pub struct Parser {
    pub toks: TokenStream,
    pub idx: usize,
}

impl Parser {
    fn new(toks: TokenStream) -> Self {
        Self { toks, idx: 0 }
    }

    fn parse(mut self) -> Ast {
        // high level pass

        let mut template = None;
        let mut document = None;
        let mut style = None;

        while self.idx < self.toks.kinds.len() {
            match self.current_token_kind() {
                TokenKind::Template => {
                    self.expect(TokenKind::Template);
                    self.expect(TokenKind::LeftBrace);
                    let template_block = self.parse_template_block();
                    template = Some(TemplateBlock {
                        statements: template_block,
                    });
                }
                TokenKind::Document => {
                    self.expect(TokenKind::Document);
                    self.expect(TokenKind::LeftBrace);
                    let document_block = self.parse_document_block();
                    document = Some(DocumentBlock {
                        elements: document_block,
                    });
                }
                TokenKind::Style => {
                    self.expect(TokenKind::Style);
                    let style_block = vec![];
                    self.skip_optional_block(); // TODO
                    style = Some(StyleBlock {
                        statements: style_block,
                    });
                }
                TokenKind::Eof => break,
                _ => panic!(
                    "Parse error: unexpected token at top level (can only be Template, Document, Style at top level). Found: {:?} at {}:{}",
                    self.current_token_kind(),
                    self.current_token_line(),
                    self.current_token_col()
                ),
            }
        }

        Ast {
            template,
            document,
            style,
        }
    }

    pub fn parse_expression(&mut self) -> Expression {
        match self.current_token_kind() {
            TokenKind::Minus => {
                self.advance();
                let right = self.parse_expression();
                Expression::Unary {
                    operator: crate::ast::UnaryOp::Negate,
                    expression: Box::new(right),
                }
            }
            // TODO handle not operator
            TokenKind::StringLiteral => {
                let value = self.current_text();
                let trimmed = value.trim_matches('"').to_string();
                self.advance();
                Expression::StringLiteral(trimmed)
            }
            TokenKind::Float => {
                let value = self.current_text();
                self.advance();
                Expression::Float(value.parse().unwrap())
            }
            TokenKind::Int => {
                let value = self.current_text();
                self.advance();
                Expression::Int(value.parse().unwrap())
            }
            TokenKind::Dollarsign => {
                // TODO not sure if this is necessary anymore
                self.advance(); // first $
                let expression = self.parse_expression();
                self.advance(); // other $
                expression
            }
            TokenKind::Identifier => self.parse_binary_expr(),
            _ => panic!(
                "Parse error: unexpected token parsing expression. Found: {:?} at {}:{}",
                self.current_token_kind(),
                self.current_token_line(),
                self.current_token_col()
            ),
        }
    }

    fn parse_binary_expr(&mut self) -> Expression {
        let left = match self.current_token_kind() {
            TokenKind::Identifier => {
                let name = self.current_text();
                self.advance();
                Expression::Identifier(name)
            }
            _ => panic!(
                "Parse error: unexpected token in binary expression {:?} at {}:{}",
                self.current_token_kind(),
                self.current_token_line(),
                self.current_token_col()
            ),
        };

        while let TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::Equals = self.current_token_kind()
        {
            let operator = match self.current_token_kind() {
                TokenKind::Plus => crate::ast::BinaryOp::Add,
                TokenKind::Minus => crate::ast::BinaryOp::Subtract,
                TokenKind::Star => crate::ast::BinaryOp::Multiply,
                TokenKind::Slash => crate::ast::BinaryOp::Divide,
                TokenKind::Equals => crate::ast::BinaryOp::Equals,
                _ => unreachable!(),
            };
            self.advance(); // consume operator
            let right = self.parse_expression();
            return Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }
}
