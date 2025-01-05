use crate::{
    evaluation::{
        ContextRef, EvaluatedExpression, EvaluatedExpressionResult, Expression, ValueType,
    },
    token::Token,
};

pub fn handle_function_declaration(
    context: &mut ContextRef,
    name: &Token,
    args: &Vec<String>,
    body: &Box<Expression>,
) -> Result<EvaluatedExpressionResult, String> {
    context.borrow_mut().set_declaration(
        name.token_type.get_lexeme(),
        EvaluatedExpression {
            value_type: ValueType::FUNCTION {
                name: name.token_type.get_lexeme(),
                params: args.clone(),
                body: body.clone(),
                context: context.clone(),
            },
        },
    );
    Ok(EvaluatedExpression::nil().into())
}
