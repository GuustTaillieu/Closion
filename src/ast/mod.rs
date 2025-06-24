use crate::ast::lexer::Token;

pub mod lexer;
pub mod parser;
pub mod evaluator;
pub mod diagnostics;

pub struct Ast {
    statements: Vec<AstStatement>,
}

pub enum AstStatementKind {
    Expression(AstExpression),
    VariableDeclaration,
    FunctionDeclaration,
    IfStatement,
    WhileStatement,
    ForStatement,
    ReturnStatement,
}

pub enum AstExpressionKind {
    Number(
        AstNumberExpression
    ),
    Binary(
        AstBinaryExpression
    ),
    Parenthesis(
        AstParenthesisExpression
    ),
}

#[derive(Debug)]
pub enum AstBinaryOperatorKind {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct AstNumberExpression {
    number: i64,
}

pub struct AstBinaryExpression {
    left: Box<AstExpression>,
    operator: AstBinaryOperator,
    right: Box<AstExpression>,
}

pub struct AstParenthesisExpression {
    expression: Box<AstExpression>,
}

pub struct AstStatement {
    kind: AstStatementKind
}


pub struct AstExpression {
    kind: AstExpressionKind,
}

pub struct AstPrinter {
    indent: usize,
}

pub struct AstBinaryOperator {
    kind: AstBinaryOperatorKind,
    token: Token
}


impl Ast {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }

    pub fn add_statement(&mut self, statement: AstStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&self, visitor: &mut dyn AstVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }

    pub fn visualize(&self) {
        let mut printer = AstPrinter::new(1);
        self.visit(&mut printer);
    }
}

pub trait AstVisitor {
    fn do_visit_statement(&mut self, statement: &AstStatement) {
        match &statement.kind {
            AstStatementKind::Expression(expr) => self.visit_expression(expr),
            AstStatementKind::VariableDeclaration => {}
            AstStatementKind::FunctionDeclaration => {}
            AstStatementKind::IfStatement => {}
            AstStatementKind::WhileStatement => {}
            AstStatementKind::ForStatement => {}
            AstStatementKind::ReturnStatement => {}
        }
    }

    fn visit_statement(&mut self, statement: &AstStatement) {
        self.do_visit_statement(statement);
    }

    fn do_visit_expression(&mut self, expression: &AstExpression) {
        match &expression.kind {
            AstExpressionKind::Number(number) => self.visit_number(number),
            AstExpressionKind::Binary(expr) => self.visit_binary_expression(&expr),
            AstExpressionKind::Parenthesis(expr) => self.visit_parenthesis_expression(&expr),
        }
    }

    fn visit_expression(&mut self, expression: &AstExpression) {
        self.do_visit_expression(expression);
    }

    fn visit_number(&mut self, number_expression: &AstNumberExpression);

    fn visit_binary_expression(&mut self, binary_expression: &AstBinaryExpression) {
            self.visit_expression(&binary_expression.left);
            self.visit_expression(&binary_expression.right);
    }
    
    fn visit_parenthesis_expression(&mut self, parenthesis_expression: &AstParenthesisExpression) {
        self.visit_expression(&parenthesis_expression.expression);
    }
}

const INDENT_LEVEL: usize = 2;

impl AstVisitor for AstPrinter {
    fn visit_statement(&mut self, statement: &AstStatement)  {
        self.print_with_indent(self.indent, "Statement");
        self.indent += INDENT_LEVEL;
        self.do_visit_statement(statement);
        self.indent -= INDENT_LEVEL;
    }

    fn visit_expression(&mut self, expression: &AstExpression) {
        self.print_with_indent(self.indent, "Expression");
        self.indent += INDENT_LEVEL;
        self.do_visit_expression(expression);
        self.indent -= INDENT_LEVEL;
    }

    fn visit_number(&mut self, number_expression: &AstNumberExpression) {
         self.print_with_indent(self.indent, &format!("Number: {}", number_expression.number));
    }

    fn visit_binary_expression(&mut self, binary_expression: &AstBinaryExpression) {
        self.print_with_indent(self.indent, "Binary Expression");
        self.indent += INDENT_LEVEL;
        self.print_with_indent(self.indent, &format!("Operator: {:?}", binary_expression.operator.kind));
        self.visit_expression(&binary_expression.left);
        self.visit_expression(&binary_expression.right);
        self.indent -= INDENT_LEVEL;
    }

    fn visit_parenthesis_expression(&mut self, parenthesis_expression: &AstParenthesisExpression) {
        self.print_with_indent(self.indent, "Parenthesis Expression");
        self.indent += INDENT_LEVEL;
        self.visit_expression(&parenthesis_expression.expression);
        self.indent -= INDENT_LEVEL;
    }
}

impl AstPrinter {
    pub fn new(indent: usize) -> Self {
        Self { indent }
    }

    fn print_with_indent(&self, indent: usize, text: &str) {
        println!("{}{}", " ".repeat(indent), text);
    }
}

impl AstExpression {
    pub fn new(kind: AstExpressionKind) -> Self {
        Self { kind }
    }

    pub fn number(number: i64) -> Self {
        Self::new(AstExpressionKind::Number(AstNumberExpression{ number }))
    }

    pub fn binary(
        left: AstExpression,
        operator: AstBinaryOperator,
        right: AstExpression,
    ) -> Self {
        Self {
            kind: AstExpressionKind::Binary(AstBinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }),
        }
    }

    pub fn parenthesis(expression: AstExpression) -> Self {
        Self {
            kind: AstExpressionKind::Parenthesis(
                AstParenthesisExpression {
                    expression: Box::new(expression),
                }
            ),
        }
    }
}

impl AstStatement {
    pub fn new(kind: AstStatementKind) -> Self {
        Self { kind }
    }

    pub fn expression(expr: AstExpression) -> Self {
        Self::new(AstStatementKind::Expression(expr))
    }
}

impl AstBinaryOperator {
    pub fn new(kind: AstBinaryOperatorKind, token: Token) -> Self {
        Self { kind, token }
    }
    
    pub fn precedence(&self) -> u8 {
        match self.kind {
            AstBinaryOperatorKind::Add => 1,
            AstBinaryOperatorKind::Subtract => 1,
            AstBinaryOperatorKind::Multiply => 2,
            AstBinaryOperatorKind::Divide => 2,
        }
    }
}