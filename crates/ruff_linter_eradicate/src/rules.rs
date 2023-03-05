use rustpython_parser::ast::Location;

use ruff_linter::diagnostic::Diagnostic;
use ruff_linter::fix::Fix;
use ruff_linter::range::Range;
use ruff_linter::violation::AlwaysAutofixableViolation;
use ruff_macros::{define_violation, derive_message_formats};
use ruff_python_ast::source_code::Locator;

use super::detection::comment_contains_code;

define_violation!(
    /// ## What it does
    /// Checks for commented-out Python code.
    ///
    /// ## Why is this bad?
    /// Commented-out code is dead code, and is often included inadvertently.
    /// It should be removed.
    ///
    /// ## Example
    /// ```python
    /// # print('foo')
    /// ```
    pub struct CommentedOutCode;
);
impl AlwaysAutofixableViolation for CommentedOutCode {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Found commented-out code")
    }

    fn autofix_title(&self) -> String {
        "Remove commented-out code".to_string()
    }
}

fn is_standalone_comment(line: &str) -> bool {
    for char in line.chars() {
        if char == '#' {
            return true;
        } else if !char.is_whitespace() {
            return false;
        }
    }
    unreachable!("Comment should contain '#' character")
}

/// ERA001
pub fn commented_out_code(
    locator: &Locator,
    start: Location,
    end: Location,
    task_tags: &[String],
    autofix: bool,
) -> Option<Diagnostic> {
    let location = Location::new(start.row(), 0);
    let end_location = Location::new(end.row() + 1, 0);
    let line = locator.slice(&Range::new(location, end_location));

    // Verify that the comment is on its own line, and that it contains code.
    if is_standalone_comment(line) && comment_contains_code(line, task_tags) {
        let mut diagnostic = Diagnostic::new(CommentedOutCode, Range::new(start, end));
        if autofix {
            diagnostic.amend(Fix::deletion(location, end_location));
        }
        Some(diagnostic)
    } else {
        None
    }
}
