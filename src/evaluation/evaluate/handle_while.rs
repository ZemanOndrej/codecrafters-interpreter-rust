use crate::evaluation::{ContextRef, EvaluatedExpression, EvaluatedExpressionResult, Expression};

pub fn handle_while(
    context: &mut ContextRef,
    condition: &Expression,
    then: &Expression,
) -> Result<EvaluatedExpressionResult, String> {
    loop {
        let condition = condition.evaluate(context)?.assert_value()?;
        // dbg!(&condition);

        if !condition.to_bool() {
            break;
        }
        let res = then.evaluate(context)?;
        if matches!(res, EvaluatedExpressionResult::FunctionReturn(_)) {
            return Ok(res);
        }
    }
    Ok(EvaluatedExpression::nil().into())
}
