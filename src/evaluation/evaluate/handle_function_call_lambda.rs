use crate::evaluation::{ContextRef, EvaluatedExpressionResult, Expression, ValueType};

use super::eval_args;

pub fn handle_function_call_lambda(
    context: &mut ContextRef,
    expr: &Expression,
    args: &Vec<Expression>,
) -> Result<EvaluatedExpressionResult, String> {
    let function = expr.evaluate(context)?.assert_value()?;

    let ValueType::FUNCTION {
        params: fn_args,
        body,
        context: closure,
        ..
    } = function.value_type
    else {
        return Err(format!("Not a function type."));
    };

    let mut child_context = eval_args(context, args, fn_args, closure, "lambda")?;
    // dbg!(&child_context.borrow());
    // dbg!(closure.borrow());

    let result = body.evaluate(&mut child_context);
    return result.map(|v| match v {
        EvaluatedExpressionResult::FunctionReturn(value) => value.into(),
        r => r,
    });
}
