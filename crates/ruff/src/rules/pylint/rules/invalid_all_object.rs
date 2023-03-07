use rustpython_parser::ast::Expr;

use ruff_macros::{derive_message_formats, violation};

use crate::registry::Diagnostic;
use crate::violation::Violation;

#[violation]
pub struct InvalidAllObject;

impl Violation for InvalidAllObject {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Invalid object in `__all__`, must contain only strings")
    }
}

/// PLE0604
pub fn invalid_all_object(expr: &Expr) -> Diagnostic {
    Diagnostic::new(InvalidAllObject, expr.into())
}
