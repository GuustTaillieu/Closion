use crate::ast::{AstBinaryOperator, AstBinaryOperatorKind, AstExpression, AstExpressionKind, AstNumberExpression, AstStatement};
use crate::ast::lexer::{Lexer, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(
        tokens: Vec<Token>,
    ) -> Self {
        let filtered_tokens = tokens.into_iter()
            .filter(|token| token.kind != TokenKind::Whitespace)
            .collect();
        Self{ tokens: filtered_tokens, current: 0 }
    }

    pub fn from_input(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }
        Self::new(tokens)
    }

    pub fn next_statement(&mut self) -> Option<AstStatement> {
        self.parse_statement()
    }

    fn parse_statement(&mut self) -> Option<AstStatement> {
        let token = self.current()?;
        if token.kind == TokenKind::EOF {
            return None;
        }
        let expr = self.parse_expression()?;
        Some(AstStatement::expression(expr))
    }
    
    fn parse_expression(&mut self) -> Option<AstExpression> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<AstExpression> {
        let mut left = self.parse_primary_expression()?;
        while let Some(operator) = self.parse_operator() {
            self.consume();
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }
            let right = self.parse_binary_expression(operator_precedence)?;
            left = AstExpression::binary(left, operator, right);
        }
        Some(left)
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

    fn parse_primary_expression(&mut self) -> Option<AstExpression> {
        let token = self.consume()?;
        match token.kind {
            TokenKind::Number(number) => Some(AstExpression::new(AstExpressionKind::Number(AstNumberExpression {number}))),
            TokenKind::LParen => {
                let expr = self.parse_expression()?;
                let token = self.consume()?;
                if token.kind != TokenKind::RParen {
                    return None;
                }
                Some(AstExpression::parenthesis(expr))
            }
            _ => None,
        }
    }
        
    fn peek(&self, offset: isize) -> Option<&Token> {
        self.tokens.get((self.current as isize + offset) as usize)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.current += 1;
        let token = self.peek(-1)?;
        Some(token)
    }
}