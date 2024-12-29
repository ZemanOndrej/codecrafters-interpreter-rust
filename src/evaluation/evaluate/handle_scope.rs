use crate::evaluation::{
    Context, ContextRef, EvaluatedExpression, EvaluatedExpressionResult, Expression,
};

pub fn handle_scope(
    context: &mut ContextRef,
    exprs: &Vec<Expression>,
) -> Result<EvaluatedExpressionResult, String> {
    let mut child_context = Context::new(context.clone());
    // dbg!(exprs);
    for expr in exprs {
        let res = expr.evaluate(&mut child_context)?;
        if matches!(res, EvaluatedExpressionResult::FunctionReturn(_)) {
            return Ok(res);
        }
    }
    Ok(EvaluatedExpression::nil().into())
}
