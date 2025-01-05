use crate::evaluation::{ContextRef, EvaluatedExpressionResult, Expression, ValueType};

pub fn handle_return(
    context: &mut ContextRef,
    expr: &Box<Expression>,
) -> Result<EvaluatedExpressionResult, String> {
    dbg!(expr);
    let res = expr.evaluate(context)?.assert_value()?;

    // set latest closure context for returned function
    let res = match res.value_type {
        ValueType::FUNCTION {
            name,
            params,
            body,
            context: _,
        } => ValueType::FUNCTION {
            name,
            params,
            body,
            context: context.clone(),
        },
        other => other,
    };
    Ok(EvaluatedExpressionResult::FunctionReturn(res.into()))
}
