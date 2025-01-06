mod handle_binary;
mod handle_for;
mod handle_function_call;
mod handle_function_call_lambda;
mod handle_function_declaration;
mod handle_if_else;
mod handle_literal;
mod handle_return;
mod handle_scope;
mod handle_unary;
mod handle_variable;
mod handle_while;

use handle_binary::handle_binary;
use handle_for::handle_for;
use handle_function_call::{eval_args, handle_function_call};
use handle_function_call_lambda::handle_function_call_lambda;
use handle_function_declaration::handle_function_declaration;
use handle_if_else::handle_if_else;
use handle_literal::handle_literal;
use handle_return::handle_return;
use handle_scope::handle_scope;
use handle_unary::handle_unary;
use handle_variable::handle_variable;
use handle_while::handle_while;

use super::{Context, ContextRef, EvaluatedExpressionResult, Expression};

impl Expression {
    pub fn evaluate(&self, context: &mut ContextRef) -> Result<EvaluatedExpressionResult, String> {
        use Expression::*;
        match self {
            Literal(t) => handle_literal(context, t),
            Binary(expression, token, expression1) => {
                handle_binary(context, expression, token, expression1)
            }
            Unary(token, expression) => handle_unary(context, token, expression),
            Variable(name, expr) => handle_variable(context, name, expr),
            Grouping(expression) => expression.evaluate(context),
            FunctionCall(t, args) => handle_function_call(context, t, args),
            FunctionCallLambda(expr, args) => handle_function_call_lambda(context, expr, args),
            FunctionDeclaration { name, args, body } => {
                handle_function_declaration(context, name, args, body)
            }
            Return(expr) => handle_return(context, expr),
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
