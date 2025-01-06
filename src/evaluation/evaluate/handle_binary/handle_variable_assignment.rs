use crate::{
    evaluation::{ContextRef, EvaluatedExpression, EvaluatedExpressionResult, Expression},
    token_type::TokenType,
};

pub fn handle_variable_assignment(
    context: &mut ContextRef,
    expression: &Expression,
    right: &EvaluatedExpression,
) -> Option<Result<EvaluatedExpressionResult, String>> {
    use Expression::*;
    use TokenType::*;

    match expression {
        Literal(t) => {
            if let IDENTIFIER(identifier) = &t.token_type {
                if !context.borrow().contains_declaration(identifier) {
                    return Some(Err(format!(
                        "Undefined variable '{}'.\n[line {}]",
                        identifier, t.line_index
                    )));
                }
                context
                    .borrow_mut()
                    .change_declaration(identifier, right.clone());
                return Some(Ok(right.clone().into()));
            }
        }
        Binary(e1, t, e2) => {
            dbg!(e1, t, e2);
        }
        _ => (),
    }
    None
}
