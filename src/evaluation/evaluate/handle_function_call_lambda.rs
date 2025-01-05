use crate::evaluation::{Context, ContextRef, EvaluatedExpressionResult, Expression, ValueType};

pub fn handle_function_call_lambda(
    context: &mut ContextRef,
    expr: &Expression,
    args: &Vec<Expression>,
) -> Result<EvaluatedExpressionResult, String> {
    let function = expr.evaluate(context)?.assert_value()?;

    let ValueType::FUNCTION {
        params: fn_args,
        body,
        context: _,
        ..
    } = function.value_type
    else {
        return Err(format!("Not a function type."));
    };

    let mut child_context = Context::new(context.clone());

    for (i, arg) in args.iter().enumerate() {
        let value = arg.evaluate(&mut child_context)?.assert_value()?;
        let Some(arg) = fn_args.get(i) else {
            return Err(format!("Bad arguments for function."));
        };

        child_context
            .borrow_mut()
            .set_declaration(arg.to_string(), value.into());
    }
    let result = body.evaluate(&mut child_context);
    return result.map(|v| match v {
        EvaluatedExpressionResult::FunctionReturn(value) => value.into(),
        r => r,
    });
}
