use crate::evaluation::{
    ContextRef, EvaluatedExpression, EvaluatedExpressionResult, Expression, ValueType,
};

pub fn handle_if_else(
    context: &mut ContextRef,
    condition: &Box<Expression>,
    then: &Box<Expression>,
    else_expr: &Option<Box<Expression>>,
) -> Result<EvaluatedExpressionResult, String> {
    let condition = condition.evaluate(context)?.assert_value()?;
    if condition.to_bool() {
        then.evaluate(context)
    } else {
        if let Some(expr) = else_expr {
            expr.evaluate(context)
        } else {
            Ok(EvaluatedExpression {
                value: "".to_string(),
                value_type: ValueType::NIL,
            }
            .into())
        }
    }
}
