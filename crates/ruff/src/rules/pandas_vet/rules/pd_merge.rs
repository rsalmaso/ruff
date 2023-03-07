use ruff_macros::{derive_message_formats, violation};
use rustpython_parser::ast::{Expr, ExprKind};

use crate::registry::Diagnostic;
use crate::violation::Violation;

#[violation]
pub struct UseOfPdMerge;

impl Violation for UseOfPdMerge {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Use `.merge` method instead of `pd.merge` function. They have equivalent \
             functionality."
        )
    }
}

/// PD015
pub fn use_of_pd_merge(func: &Expr) -> Option<Diagnostic> {
    if let ExprKind::Attribute { attr, value, .. } = &func.node {
        if let ExprKind::Name { id, .. } = &value.node {
            if id == "pd" && attr == "merge" {
                return Some(Diagnostic::new(UseOfPdMerge, func.into()));
            }
        }
    }
    None
}
