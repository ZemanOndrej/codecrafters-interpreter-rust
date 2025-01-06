use crate::evaluation::{ContextRef, EvaluatedExpression, Expression};

pub fn print(args: &[Expression], context: &mut ContextRef) -> Result<EvaluatedExpression, String> {
    let first_argument = args.first().ok_or("Missing argument")?;
    let value = first_argument
        .evaluate(context)?
        .assert_value()?
        .to_string();

    println!("{}", value);

    Ok(EvaluatedExpression::nil())
}
