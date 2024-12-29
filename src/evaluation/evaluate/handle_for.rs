use crate::evaluation::{ContextRef, EvaluatedExpression, EvaluatedExpressionResult, Expression};

pub fn handle_for(
    context: &mut ContextRef,
    declaration: &Box<Expression>,
    condition: &Box<Expression>,
    increment: &Box<Expression>,
    then: &Box<Expression>,
) -> Result<EvaluatedExpressionResult, String> {
    declaration.evaluate(context)?;
    loop {
        let condition = condition.evaluate(context)?.assert_value()?;
        if !condition.to_bool() {
            break;
        }
        let res = then.evaluate(context)?;
        if matches!(res, EvaluatedExpressionResult::FunctionReturn(_)) {
            return Ok(res);
        }
        increment.evaluate(context)?;
    }
    Ok(EvaluatedExpression::nil().into())
}
