use crate::evaluate::{EvaluatedExpression, Expression, ValueType};

pub fn print(args: &Vec<Expression>) -> Result<EvaluatedExpression, String> {
    let value = args.get(0).ok_or("Missing argument")?.evaluate()?.value;
    dbg!(&value);
    print!("{}", value);

    Ok(EvaluatedExpression {
        value: "".to_string(),
        value_type: ValueType::NIL,
    })
}
