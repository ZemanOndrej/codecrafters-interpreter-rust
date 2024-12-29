mod clock;
mod print;

use std::collections::HashMap;

pub use clock::*;
pub use print::*;

use crate::evaluation::{ContextRef, EvaluatedExpression, Expression};

pub fn get_builtin_fns() -> HashMap<&'static str, BuiltinFn<'static>> {
    let mut builtin_fns = HashMap::new();

    builtin_fns.insert(CLOCK.name, CLOCK);

    builtin_fns
}

pub struct BuiltinFn<'a> {
    pub name: &'a str,
    pub function:
        fn(args: &Vec<Expression>, context: &mut ContextRef) -> Result<EvaluatedExpression, String>,
}
