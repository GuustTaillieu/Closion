use crate::ast::{AstBinaryOperator, AstBinaryOperatorKind, AstExpression, AstExpressionKind, AstNumberExpression, AstStatement};
use crate::ast::diagnostics::{DiagnosticBagCell, DiagnosticsBag};
use crate::ast::lexer::{Lexer, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    diagnostic_bag: DiagnosticBagCell,
}

impl Parser {
    pub fn new(
        tokens: Vec<Token>,
        diagnostic_bag: DiagnosticBagCell
    ) -> Self {
        let filtered_tokens = tokens.into_iter()
            .filter(|token| token.kind != TokenKind::Whitespace)
            .collect();
        Self{ tokens: filtered_tokens, current: 0, diagnostic_bag }
    }

    pub fn from_input(input: &str, diagnostics_bag: DiagnosticBagCell) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        Self::new(tokens, diagnostics_bag)
    }

    pub fn next_statement(&mut self) -> AstStatement {
        self.parse_statement()
    }

    fn parse_statement(&mut self) -> AstStatement {
        let token = self.current();
        if token.kind == TokenKind::EOF {
            return None;
        }
        let expr = self.parse_expression();
        AstStatement::expression(expr)
    }

    fn parse_expression(&mut self) -> AstExpression {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> AstExpression {
        let mut left = self.parse_primary_expression();
        while let Some(operator) = self.parse_operator() {
            self.consume();
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }
            let right = self.parse_binary_expression(operator_precedence);
            left = AstExpression::binary(left, operator, right);
        }
        left
    }

    fn parse_operator(&mut self) -> Option<AstBinaryOperator> {
        let token = self.current()?;
        let kind = match token.kind {
            TokenKind::Plus => Some(AstBinaryOperatorKind::Add),
            TokenKind::Minus => Some(AstBinaryOperatorKind::Subtract),
            TokenKind::Asterisk => Some(AstBinaryOperatorKind::Multiply),
            TokenKind::Slash => Some(AstBinaryOperatorKind::Divide),
            _ => None,
        };
        kind.map(|kind| AstBinaryOperator::new(kind, token.clone()))
    }

    fn parse_primary_expression(&mut self) -> AstExpression {
        let token = self.consume();
        match token.kind {
            TokenKind::Number(number) => AstExpression::new(AstExpressionKind::Number(AstNumberExpression {number})),
            TokenKind::LParen => {
                let expr = self.parse_expression();
                let token = self.consume();
                if token.kind != TokenKind::RParen {
                    return None;
                }
                AstExpression::parenthesis(expr)
            }
            _ => todo!("Add error"),
        }
    }
        
    fn peek(&self, offset: isize) -> &Token {
        let mut index = (self.current as isize + offset) as usize;
        if index >= self.tokens.len() {
            index = self.tokens.len() - 1;
        }
        self.tokens.get(index).unwrap()
    }

    fn current(&self) -> &Token {
        self.peek(0)
    }

    fn consume(&mut self) -> &Token {
        self.current += 1;
        self.peek(-1)
    }
}