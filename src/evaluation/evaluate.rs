mod handle_binary;
mod handle_for;
mod handle_function_call;
mod handle_if_else;
mod handle_literal;
mod handle_scope;
mod handle_unary;
mod handle_while;

use handle_binary::*;
use handle_for::*;
use handle_function_call::*;
use handle_if_else::*;
use handle_literal::*;
use handle_scope::*;
use handle_unary::*;
use handle_while::*;

use super::{Context, ContextRef, EvaluatedExpression, EvaluatedExpressionResult, Expression};
use crate::evaluation::ValueType;

impl Expression {
    pub fn evaluate(&self, context: &mut ContextRef) -> Result<EvaluatedExpressionResult, String> {
        use Expression::*;

        match self {
            Literal(t) => handle_literal(context, t),
            Binary(expression, token, expression1) => {
                handle_binary(context, expression, token, expression1)
            }
            Unary(token, expression) => handle_unary(context, token, expression),
            Variable(name, expr) => {
                let evaluated_expression_result = expr.evaluate(context)?;
                let value = evaluated_expression_result.assert_value()?;
                context
                    .borrow_mut()
                    .set_declaration(name.clone(), value.into());
                Ok(EvaluatedExpression::nil().into())
            }
            Grouping(expression) => expression.evaluate(context),
            FunctionCall(t, args) => handle_function_call(context, t, args),
            FunctionDeclaration { name, args, body } => {
                context.borrow_mut().set_declaration(
                    name.token_type.get_lexeme(),
                    EvaluatedExpression {
                        value_type: ValueType::FUNCTION {
                            name: name.token_type.get_lexeme(),
                            params: args.clone(),
                            body: body.clone(),
                        },
                    },
                );
                Ok(EvaluatedExpression::nil().into())
            }
            Return(expr) => {
                dbg!(expr);
                let res = expr
                    .evaluate(context)?
                    .assert_value()
                    .map(|v| EvaluatedExpressionResult::FunctionReturn(v));
                res
            }
            Scope(_, exprs) => handle_scope(context, exprs),
            IfElse {
                condition,
                then,
                else_expr,
            } => handle_if_else(context, condition, then, else_expr),
            For {
                declaration,
                condition,
                increment,
                then,
            } => handle_for(context, declaration, condition, increment, then),
            While { condition, then } => handle_while(context, condition, then),
        }
    }
}
