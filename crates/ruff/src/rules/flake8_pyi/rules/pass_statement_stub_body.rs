use rustpython_parser::ast::{Stmt, StmtKind};

use ruff_macros::{derive_message_formats, violation};

use crate::checkers::ast::Checker;
use crate::registry::Diagnostic;
use crate::violation::Violation;

#[violation]
pub struct PassStatementStubBody;

impl Violation for PassStatementStubBody {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Empty body should contain `...`, not `pass`")
    }
}

/// PYI009
pub fn pass_statement_stub_body(checker: &mut Checker, body: &[Stmt]) {
    if body.len() != 1 {
        return;
    }
    let stmt = &body[0];
    if matches!(stmt.node, StmtKind::Pass) {
        checker
            .diagnostics
            .push(Diagnostic::new(PassStatementStubBody, stmt.into()));
    }
}
