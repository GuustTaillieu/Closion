use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::lexer::{TextSpan, TokenKind};

pub enum DiagnosticKind {
    Error,
    Warning,
}

pub struct Diagnostic {
    pub message: String,
    pub span: TextSpan,
    pub kind: DiagnosticKind,
}

pub struct DiagnosticsBag {
    pub diagnostics: Vec<Diagnostic>,
}

pub type  DiagnosticBagCell = Rc<RefCell<DiagnosticsBag>>;

impl Diagnostic {
    pub fn new(
        message: String,
        span: TextSpan,
        kind: DiagnosticKind,
    ) -> Self {
        Self { message, span, kind }
    }

    pub fn error(message: String, span: TextSpan) -> Self {
        Self::new(message, span, DiagnosticKind::Error)
    }

    pub fn warning(message: String, span: TextSpan) -> Self {
        Self::new(message, span, DiagnosticKind::Warning)
    }
}

impl DiagnosticsBag {
    pub fn new() -> Self {
        Self { diagnostics: Vec::new() }
    }

    pub fn report_error(
        &mut self,
        message: String,
        span: TextSpan,
    ) {
        let diagnostic = Diagnostic::error(message, span);
        self.diagnostics.push(diagnostic);
    }
    
    pub fn report_warning(
        &mut self,
        message: String,
        span: TextSpan,
    ) {
        let diagnostic = Diagnostic::warning(message, span);
        self.diagnostics.push(diagnostic);
    }
    
    pub fn report_unexpected_token(
        &mut self,
        expected: &TokenKind,
        found: &TokenKind,
        span: TextSpan,
    ) {
        let message = format!("Expected <{}>, but found <{}>", expected, found);
        self.report_error(message, span);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| matches!(d.kind, DiagnosticKind::Error))
    }

    pub fn has_warnings(&self) -> bool {
        self.diagnostics.iter().any(|d| matches!(d.kind, DiagnosticKind::Warning))
    }
}