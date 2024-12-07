use crate::{
    evaluate::{EvaluatedExpression, Expression, ValueType},
    handlers::Context,
};

pub fn print(args: &Vec<Expression>, context: &mut Context) -> Result<EvaluatedExpression, String> {
    let first_argument = args.get(0).ok_or("Missing argument")?;
    let value = first_argument.evaluate(context)?.value;

    println!("{}", value);

    Ok(EvaluatedExpression {
        value: "".to_string(),
        value_type: ValueType::NIL,
    })
}
