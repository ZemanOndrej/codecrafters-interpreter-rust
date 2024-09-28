use crate::parse::{EvaluatedExpression, Expression, ValueType};

pub fn print(args: &Vec<Expression>) -> Result<EvaluatedExpression, String> {
    println!("{}", args.get(0).unwrap().to_string());

    Ok(EvaluatedExpression {
        value: "".to_string(),
        value_type: ValueType::NIL,
    })
}
