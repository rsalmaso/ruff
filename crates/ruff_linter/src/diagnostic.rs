use rustpython_parser::ast::Location;
use serde::{Deserialize, Serialize};

use crate::fix::Fix;
use crate::range::Range;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticKind {
    /// The identifier of the corresponding [`Rule`].
    pub rule: String,
    /// The message body to display to the user, to explain the diagnostic.
    pub body: String,
    /// The message to display to the user, to explain the suggested fix.
    pub commit: Option<String>,
    /// Whether the diagnostic is automatically fixable.
    pub fixable: bool,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub location: Location,
    pub end_location: Location,
    pub fix: Option<Fix>,
    pub parent: Option<Location>,
}

impl Diagnostic {
    pub fn new<K: Into<DiagnosticKind>>(kind: K, range: Range) -> Self {
        Self {
            kind: kind.into(),
            location: range.location,
            end_location: range.end_location,
            fix: None,
            parent: None,
        }
    }

    pub fn amend(&mut self, fix: Fix) -> &mut Self {
        self.fix = Some(fix);
        self
    }

    pub fn parent(&mut self, parent: Location) -> &mut Self {
        self.parent = Some(parent);
        self
    }
}
