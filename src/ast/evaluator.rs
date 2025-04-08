use crate::ast::{AstBinaryExpression, AstBinaryOperatorKind, AstExpression, AstNumberExpression, AstVisitor};

pub struct AstEvaluator {
    pub last_value: Option<i64>,
}

impl AstVisitor for AstEvaluator {
    fn visit_number(&mut self, number_expression: &AstNumberExpression) {
        self.last_value = Some(number_expression.number)
    }
    
    fn visit_binary_expression(&mut self, binary_expression: &AstBinaryExpression) {
        self.visit_expression(&binary_expression.left);
        let left_value = self.last_value.unwrap();
        self.visit_expression(&binary_expression.right);
        let right_value = self.last_value.unwrap();
        let result = match binary_expression.operator.kind {
            AstBinaryOperatorKind::Add => left_value + right_value,
            AstBinaryOperatorKind::Subtract => left_value - right_value,
            AstBinaryOperatorKind::Multiply => left_value * right_value,
            AstBinaryOperatorKind::Divide => left_value / right_value,
        };
        self.last_value = Some(result);
    }
}

impl AstEvaluator {
    pub fn new() -> Self {
        Self { last_value: None }
    }
}