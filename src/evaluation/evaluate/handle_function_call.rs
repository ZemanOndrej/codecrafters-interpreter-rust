use crate::{
    builtin_fns,
    evaluation::{Context, ContextRef, EvaluatedExpressionResult, Expression, ValueType},
    token::Token,
    token_type::TokenType,
};

pub fn handle_function_call(
    context: &mut ContextRef,
    t: &Token,
    args: &Vec<Expression>,
) -> Result<EvaluatedExpressionResult, String> {
    use TokenType::*;

    let builtin_fns = builtin_fns::get_builtin_fns();
    let fn_name = t.token_type.get_lexeme();
    if let Some(builtin_fn) = builtin_fns.get(fn_name.as_str()) {
        return (builtin_fn.function)(args, context).map(|v| v.into());
    }
    // dbg!(&context);
    if let Some(function) = context.borrow().get_declaration(fn_name.as_str()) {
        let ValueType::FUNCTION {
            params: fn_args,
            body,
            context: closure,
            ..
        } = function.value_type
        else {
            return Err(format!("Not a function '{}'", t.token_type.get_lexeme()));
        };
        let mut child_context = eval_args(
            &mut context.clone(),
            args,
            fn_args,
            closure,
            t.token_type.get_lexeme().as_str(),
        )?;
        // dbg!(&child_context.borrow());
        // dunno if this is correct, we pass to the function only closure context, current context is not passed
        let result = body.evaluate(&mut child_context);
        return result.map(|v| match v {
            EvaluatedExpressionResult::FunctionReturn(value) => value.into(),
            r => r,
        });
    }
    match t.token_type {
        PRINT => builtin_fns::print(args, context).map(|v| v.into()),
        _ => Err(format!(
            "Undefined function '{}'",
            t.token_type.get_lexeme()
        )),
    }
}

pub(super) fn eval_args(
    context: &mut ContextRef,
    args: &Vec<Expression>,
    fn_args: Vec<String>,
    closure: ContextRef,
    fn_name: &str,
) -> Result<ContextRef, String> {
    if args.len() != fn_args.len() {
        return Err(format!("Expected {} but got {}", args.len(), fn_name));
    }
    let child_context = Context::new(closure.clone());
    for (i, arg) in args.iter().enumerate() {
        let value = arg.evaluate(&mut context.clone())?.assert_value()?;
        let arg = fn_args.get(i).unwrap();

        child_context
            .borrow_mut()
            .set_declaration(arg.to_string(), value.into());
    }
    Ok(child_context)
}
