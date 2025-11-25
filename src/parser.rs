use core::panic;

use crate::ast::{Ast, Expression, Statement};
use crate::lexer::{TokenKind, TokenStream, lex};

pub fn parse(source: &str) -> Ast {
    let tokens = lex(source);
    let p = Parser::new(tokens);
    p.parse()
}

struct Parser {
    toks: TokenStream,
    idx: usize,
}

impl Parser {
    fn new(toks: TokenStream) -> Self {
        Self { toks, idx: 0 }
    }

    fn parse(mut self) -> Ast {
        // high level pass
        use crate::ast::{DocumentBlock, StyleBlock, TemplateBlock};

        let mut template = None;
        let mut document = None;
        let mut style = None;

        while self.idx < self.toks.kinds.len() {
            match self.current_token_kind() {
                TokenKind::Template => {
                    let mut template_block = Vec::new();
                    template_block = self.parse_template_block();
                    template = Some(TemplateBlock {
                        statements: template_block,
                    });
                }
                TokenKind::Document => {
                    let mut document_block = Vec::new();
                    document_block = self.parse_document_block();
                    document = Some(DocumentBlock {
                        statements: document_block,
                    });
                }
                TokenKind::Style => {
                    let style_block = Vec::new();
                    self.advance();
                    self.skip_optional_block();
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

    fn current_token_kind(&self) -> TokenKind {
        self.toks.kinds[self.idx]
    }

    fn current_token_line(&self) -> u32 {
        self.toks.lines[self.idx]
    }

    fn current_token_col(&self) -> u32 {
        self.toks.cols[self.idx]
    }

    fn current_text(&self) -> String {
        let range = &self.toks.ranges[self.idx];
        self.toks.source[range.start..range.end].to_string()
    }

    fn advance(&mut self) -> TokenKind {
        if self.idx < self.toks.kinds.len() {
            self.idx += 1;
        }
        self.toks.kinds[self.idx - 1]
    }

    fn expect(&mut self, kind: TokenKind) -> TokenKind {
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

    fn match_kind(&mut self, kind: TokenKind) -> bool {
        if self.current_token_kind() == kind {
            self.advance();
            return true;
        }
        false
    }

    /// If a block `{ ... }` follows, skip it including nested braces.
    fn skip_optional_block(&mut self) {
        // skip optional whitespace-free tokens; if next is LeftBrace, skip until matching RightBrace
        if self.idx < self.toks.kinds.len() && self.current_token_kind() == TokenKind::LeftBrace {
            // enter block
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
        
        while let TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash | TokenKind::Equals =
            self.current_token_kind()
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

    fn parse_expression(&mut self) -> Expression {
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
                self.advance();
                Expression::Literal(value)
            }
            TokenKind::Float => {
                let value = self.current_text(); // TODO handle number types properly
                self.advance();
                Expression::Literal(value)
            }
            TokenKind::Int => {
                let value = self.current_text(); // TODO handle number types properly
                self.advance();
                Expression::Literal(value)
            }
            TokenKind::Dollarsign => {
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

    fn parse_statement(&mut self) -> Statement {
        println!("Parsing statement at token: {:?}", self.current_token_kind());
        match self.current_token_kind() {
            TokenKind::Identifier => {
                if self.toks.kinds.get(self.idx + 1) == Some(&TokenKind::LeftParen) {
                    // function call
                    let func_name = self.current_text();
                    self.advance(); // consume function name
                    self.expect(TokenKind::LeftParen);
                    let mut args: Vec<Expression> = Vec::new();
                    while self.current_token_kind() != TokenKind::RightParen {
                        let expr = self.parse_expression();
                        args.push(expr);
                        if self.current_token_kind() == TokenKind::Comma {
                            self.advance(); // consume comma
                        } else {
                            break;
                        }
                    }
                    self.expect(TokenKind::RightParen);
                    return Statement::FunctionCall {
                        name: func_name,
                        args,
                    };
                } 
                else if self.toks.kinds.get(self.idx + 1) == Some(&TokenKind::LeftBrace) {
                    // default block like "<h1>some text</h1>" works but in c synax "text { some text }"
                    let name = self.current_text();
                    self.advance(); // consume name
                    self.expect(TokenKind::LeftBrace);
                    let content = self.parse_document_default();
                    Statement::Assignment {
                        name,
                        value: content,
                    }
                } else {
                    let expr = self.parse_expression();
                    Statement::Assignment {
                        name: "defaults".to_string(),
                        value: expr,
                    }
                }
            }
            TokenKind::Let => {
                self.advance();
                // TODO handle let differently if needed
                let expr = self.parse_expression();
                Statement::Assignment {
                    name: "variable".to_string(),
                    value: expr,
                }
            }
            TokenKind::Const => {
                self.advance();
                // TODO handle const differently if needed
                let expr = self.parse_expression();
                Statement::Assignment {
                    name: "constant".to_string(),
                    value: expr,
                }
            }
            TokenKind::Return => {
                self.advance(); // consume 'return'
                if self.current_token_kind() == TokenKind::StringLiteral {
                    let value = self.current_text();
                    self.advance();
                    return Statement::Return(Expression::Literal(value));
                }
                let expr: Expression = self.parse_expression();
                Statement::Return(expr)
            }
            // TODO handle if statements
            // TODO handle for loops
            // TODO handle while loops
            _ => panic!(
                "Parse error: unexpected token parsing statement. Found: {:?} at {}:{}",
                self.current_token_kind(),
                self.current_token_line(),
                self.current_token_col()
            ),
        }
    }

    fn parse_func(&mut self) -> Statement {
        self.expect(TokenKind::Func);
        
        self.expect(TokenKind::Identifier);
        let name = self.toks.source[self.toks.ranges[self.idx - 1].clone()].to_string();

        self.expect(TokenKind::LeftParen);
        let params = self.parse_params();

        self.expect(TokenKind::LeftBrace);
        let body = self.parse_block();

        Statement::Function { name, params, body }
    }

    fn parse_params(&mut self) -> Vec<crate::ast::FunctionParam> {
        let mut params = Vec::new();
        while self.match_kind(TokenKind::Identifier) {
            let param_name = self.toks.source[self.toks.ranges[self.idx - 1].clone()].to_string();
            params.push(crate::ast::FunctionParam { name: param_name });
            self.match_kind(TokenKind::Comma);
        }
        self.expect(TokenKind::RightParen);
        params
    }

    fn parse_block(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();
        while self.idx < self.toks.kinds.len() {
            match self.current_token_kind() {
                TokenKind::RightBrace => {
                    self.expect(TokenKind::RightBrace);
                    break;
                }
                TokenKind::Eof => panic!(
                    "Parse error: unexpected end of file while parsing block at {}:{}",
                    self.current_token_line(),
                    self.current_token_col()
                ),
                _ => {
                    let statement = self.parse_statement();
                    statements.push(statement);
                }
            }
        }
        statements
    }

    fn parse_template_block(&mut self) -> Vec<Statement> {
        self.expect(TokenKind::Template);
        self.expect(TokenKind::LeftBrace);
        let mut statements: Vec<Statement> = Vec::new();
        while self.idx < self.toks.kinds.len() {
            match self.current_token_kind() {
                TokenKind::RightBrace => {
                    self.advance(); // exit block
                    break;
                }
                TokenKind::Func => {
                    let statement = self.parse_func();
                    statements.push(statement);
                }
                TokenKind::Eof => break,
                _ => {
                    let statement = self.parse_statement();
                    statements.push(statement);
                }
            }
        }
        statements
    }

    // TODO handle nested structures properly
    // TODO handle text formatting properly
    // TODO handle markdown formatting properly (bold, italics, etc.)
    // TODO handle code snippets properly
    fn parse_document_default(&mut self) -> Expression {
        let mut content = String::new();
        while self.idx < self.toks.kinds.len() {
            match self.current_token_kind() {
                TokenKind::RightBrace => {
                    self.advance(); // exit block
                    break;
                }
                TokenKind::Eof => panic!(
                    "Parse error: unexpected end of file while parsing document default at {}:{}",
                    self.current_token_line(),
                    self.current_token_col()
                ),
                _ => { 

                    content.push_str(&self.current_text());
                    content.push(' ');
                    self.advance();
                }
            }
        }
        Expression::Literal(content)
    }

    fn parse_document_block(&mut self) -> Vec<Statement> {
        self.expect(TokenKind::Document);
        self.expect(TokenKind::LeftBrace);
        let mut statements: Vec<Statement> = Vec::new();
        while self.idx < self.toks.kinds.len() {
            match self.current_token_kind() {
                TokenKind::RightBrace => {
                    self.advance(); // exit block
                    break;
                }
                TokenKind::Eof => break,
                _ => {
                    let statement = self.parse_statement();
                    statements.push(statement);
                }
            }
        }
        statements
    }
}
