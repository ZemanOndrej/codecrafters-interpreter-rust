use crate::evaluation::{ContextRef, EvaluatedExpression, EvaluatedExpressionResult, Expression};

pub fn handle_variable(
    context: &mut ContextRef,
    name: &String,
    expr: &Box<Expression>,
) -> Result<EvaluatedExpressionResult, String> {
    let evaluated_expression_result = expr.evaluate(context)?;
    let value = evaluated_expression_result.assert_value()?;
    context
        .borrow_mut()
        .set_declaration(name.clone(), value.into());
    Ok(EvaluatedExpression::nil().into())
}
