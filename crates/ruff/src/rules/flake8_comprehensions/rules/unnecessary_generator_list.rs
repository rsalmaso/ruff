use log::error;
use ruff_macros::{derive_message_formats, violation};
use rustpython_parser::ast::{Expr, ExprKind, Keyword};

use super::helpers;
use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::registry::Diagnostic;
use crate::rules::flake8_comprehensions::fixes;
use crate::violation::AlwaysAutofixableViolation;

/// ## What it does
/// Checks for unnecessary generators that can be rewritten as `list`
/// comprehensions.
///
/// ## Why is this bad?
/// It is unnecessary to use `list` around a generator expression, since
/// there are equivalent comprehensions for these types. Using a
/// comprehension is clearer and more idiomatic.
///
/// ## Examples
/// ```python
/// list(f(x) for x in foo)
/// ```
///
/// Use instead:
/// ```python
/// [f(x) for x in foo]
/// ```
#[violation]
pub struct UnnecessaryGeneratorList;

impl AlwaysAutofixableViolation for UnnecessaryGeneratorList {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Unnecessary generator (rewrite as a `list` comprehension)")
    }

    fn autofix_title(&self) -> String {
        "Rewrite as a `list` comprehension".to_string()
    }
}

/// C400 (`list(generator)`)
pub fn unnecessary_generator_list(
    checker: &mut Checker,
    expr: &Expr,
    func: &Expr,
    args: &[Expr],
    keywords: &[Keyword],
) {
    let Some(argument) = helpers::exactly_one_argument_with_matching_function("list", func, args, keywords) else {
        return;
    };
    if !checker.ctx.is_builtin("list") {
        return;
    }
    if let ExprKind::GeneratorExp { .. } = argument {
        let mut diagnostic = Diagnostic::new(UnnecessaryGeneratorList, Range::from_located(expr));
        if checker.patch((&diagnostic.kind).into()) {
            match fixes::fix_unnecessary_generator_list(checker.locator, checker.stylist, expr) {
                Ok(fix) => {
                    diagnostic.amend(fix);
                }
                Err(e) => error!("Failed to generate fix: {e}"),
            }
        }
        checker.diagnostics.push(diagnostic);
    }
}
