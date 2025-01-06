use crate::evaluation::{ContextRef, EvaluatedExpression, EvaluatedExpressionResult, Expression};

pub fn handle_for(
    context: &mut ContextRef,
    declaration: &Expression,
    condition: &Expression,
    increment: &Expression,
    then: &Expression,
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
