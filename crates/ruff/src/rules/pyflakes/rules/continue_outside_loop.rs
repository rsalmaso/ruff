use ruff_macros::{derive_message_formats, violation};
use rustpython_parser::ast::{Stmt, StmtKind};

use crate::registry::Diagnostic;
use crate::violation::Violation;

#[violation]
pub struct ContinueOutsideLoop;

impl Violation for ContinueOutsideLoop {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("`continue` not properly in loop")
    }
}

/// F702
pub fn continue_outside_loop<'a>(
    stmt: &'a Stmt,
    parents: &mut impl Iterator<Item = &'a Stmt>,
) -> Option<Diagnostic> {
    let mut allowed: bool = false;
    let mut child = stmt;
    for parent in parents {
        match &parent.node {
            StmtKind::For { orelse, .. }
            | StmtKind::AsyncFor { orelse, .. }
            | StmtKind::While { orelse, .. } => {
                if !orelse.contains(child) {
                    allowed = true;
                    break;
                }
            }
            StmtKind::FunctionDef { .. }
            | StmtKind::AsyncFunctionDef { .. }
            | StmtKind::ClassDef { .. } => {
                break;
            }
            _ => {}
        }
        child = parent;
    }

    if allowed {
        None
    } else {
        Some(Diagnostic::new(ContinueOutsideLoop, stmt.into()))
    }
}
