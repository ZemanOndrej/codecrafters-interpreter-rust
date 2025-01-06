use crate::evaluation::{ContextRef, EvaluatedExpression, EvaluatedExpressionResult, Expression};

pub fn handle_variable(
    context: &mut ContextRef,
    name: &str,
    expr: &Expression,
) -> Result<EvaluatedExpressionResult, String> {
    let evaluated_expression_result = expr.evaluate(context)?;
    let value = evaluated_expression_result.assert_value()?;
    context
        .borrow_mut()
        .set_declaration(name.to_string(), value);
    Ok(EvaluatedExpression::nil().into())
}
