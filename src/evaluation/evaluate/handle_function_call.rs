use crate::{
    builtin_fns,
    evaluation::{Context, ContextRef, EvaluatedExpressionResult, Expression},
    token::Token,
    token_type::TokenType,
};

pub fn handle_function_call(
    context: &mut ContextRef,
    t: &Token,
    args: &Vec<Expression>,
) -> Result<EvaluatedExpressionResult, String> {
    use Expression::*;
    use TokenType::*;

    let builtin_fns = builtin_fns::get_builtin_fns();
    let fn_name = t.token_type.get_lexeme();
    if let Some(builtin_fn) = builtin_fns.get(fn_name.as_str()) {
        return (builtin_fn.function)(args, context).map(|v| v.into());
    }
    if let Some(function) = context.borrow().get_function(fn_name.as_str()) {
        let mut child_context = Context::new(context.clone());
        let FunctionDeclaration {
            args: fn_args,
            body,
            ..
        } = &function
        else {
            return Err(format!(
                "Undefined function '{}'",
                t.token_type.get_lexeme()
            ));
        };
        for (i, arg) in args.iter().enumerate() {
            let value = arg.evaluate(&mut child_context)?.assert_value()?;
            let Some(arg) = fn_args.get(i) else {
                return Err(format!(
                    "Bad arguments for function '{}'",
                    t.token_type.get_lexeme()
                ));
            };

            child_context
                .borrow_mut()
                .set_variable(arg.to_string(), value);
        }
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
