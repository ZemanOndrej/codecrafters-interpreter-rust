use crate::{
    evaluate::{EvaluatedExpression, Expression, ValueType},
    handlers::Context,
};

pub fn print(args: &Vec<Expression>, context: &mut Context) -> Result<EvaluatedExpression, String> {
    let value = args
        .get(0)
        .ok_or("Missing argument")?
        .evaluate(context)?
        .value;
    // dbg!(&value);
    println!("{}", value);

    Ok(EvaluatedExpression {
        value: "".to_string(),
        value_type: ValueType::NIL,
    })
}
