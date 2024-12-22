use super::BuiltinFn;
use crate::evaluate::{ContextRef, EvaluatedExpression, Expression, ValueType};
use std::time::SystemTime;

fn clock_fn(_: &Vec<Expression>, _: &mut ContextRef) -> Result<EvaluatedExpression, String> {
    Ok(EvaluatedExpression {
        value: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string(),
        value_type: ValueType::NUMBER,
    })
}

pub const CLOCK: BuiltinFn = BuiltinFn {
    name: "clock",
    function: clock_fn,
};
