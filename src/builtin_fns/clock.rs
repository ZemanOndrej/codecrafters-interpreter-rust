use super::BuiltinFn;
use crate::evaluation::{ContextRef, EvaluatedExpression, Expression, ValueType};
use std::time::SystemTime;

fn clock_fn(_: &Vec<Expression>, _: &mut ContextRef) -> Result<EvaluatedExpression, String> {
    let value = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    Ok(EvaluatedExpression {
        value_type: ValueType::NUMBER(value as f64),
    })
}

pub const CLOCK: BuiltinFn = BuiltinFn {
    name: "clock",
    function: clock_fn,
};
