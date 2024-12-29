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
                let value = expr.evaluate(context)?.assert_value()?;
                context.borrow_mut().set_variable(name.clone(), value);
                Ok(EvaluatedExpression::nil().into())
            }
            Grouping(expression) => expression.evaluate(context),
            FunctionCall(t, args) => handle_function_call(context, t, args),
            FunctionDeclaration { name, .. } => {
                context
                    .borrow_mut()
                    .set_function(name.token_type.get_lexeme(), self.clone());
                Ok(EvaluatedExpression::nil().into())
            }
            Return(expr) => expr
                .evaluate(context)?
                .assert_value()
                .map(|v| EvaluatedExpressionResult::FunctionReturn(v)),
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
