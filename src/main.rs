use crate::ast::Ast;
use crate::ast::evaluator::AstEvaluator;
use crate::ast::lexer::Lexer;
use crate::ast::parser::Parser;

mod ast;

fn main() {
    let input = "7 - (30 + 7) * 8 / 2";

    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    // println!("{:#?}", tokens);

    let mut ast = Ast::new();
    let mut parser = Parser::from_input(input);
    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }
    
    ast.visualize();
    let mut evaluator = AstEvaluator::new();
    ast.visit(&mut evaluator);
    println!("Result: {:?}", evaluator.last_value);
}
