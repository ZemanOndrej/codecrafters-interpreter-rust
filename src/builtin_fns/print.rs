use crate::evaluation::{ContextRef, EvaluatedExpression, Expression};

pub fn print(
    args: &Vec<Expression>,
    context: &mut ContextRef,
) -> Result<EvaluatedExpression, String> {
    let first_argument = args.get(0).ok_or("Missing argument")?;
    let value = first_argument.evaluate(context)?.assert_value()?.value;

    println!("{}", value);

    Ok(EvaluatedExpression::nil())
}
