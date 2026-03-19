use crate::types::{Diagnostic, DiagnosticCode, Severity};

/// Collector for diagnostics during parsing.
#[derive(Debug, Default)]
pub struct Diagnostics {
    items: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn warn(&mut self, line_number: usize, column: Option<usize>, code: DiagnosticCode, message: impl Into<String>) {
        self.items.push(Diagnostic {
            severity: Severity::Warning,
            line_number,
            column,
            code,
            message: message.into(),
        });
    }

    pub fn error(&mut self, line_number: usize, column: Option<usize>, code: DiagnosticCode, message: impl Into<String>) {
        self.items.push(Diagnostic {
            severity: Severity::Error,
            line_number,
            column,
            code,
            message: message.into(),
        });
    }

    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
