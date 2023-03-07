use rustpython_parser::ast::{Constant, ExprKind, Stmt, StmtKind};

use ruff_macros::{derive_message_formats, violation};

use crate::checkers::ast::Checker;
use crate::registry::Diagnostic;
use crate::violation::Violation;

#[violation]
pub struct NonEmptyStubBody;

impl Violation for NonEmptyStubBody {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Function body must contain only `...`")
    }
}

/// PYI010
pub fn non_empty_stub_body(checker: &mut Checker, body: &[Stmt]) {
    if body.len() != 1 {
        return;
    }
    let stmt = &body[0];
    if let StmtKind::Expr { value } = &stmt.node {
        if let ExprKind::Constant { value, .. } = &value.node {
            if matches!(value, Constant::Ellipsis | Constant::Str(_)) {
                return;
            }
        }
    }

    checker
        .diagnostics
        .push(Diagnostic::new(NonEmptyStubBody, stmt.into()));
}
