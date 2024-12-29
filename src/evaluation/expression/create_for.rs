use super::Expression;

impl Expression {
    pub fn create_for(
        declaration: Expression,
        condition: Expression,
        increment: Expression,
        then: Expression,
    ) -> Result<Self, String> {
        use Expression::*;
        match declaration {
            Variable(_, _) | Literal(_) | Binary(_, _, _) => {}
            _ => {
                return Err("Expected variable declaration".to_string());
            }
        }
        match condition {
            Binary(_, _, _) | Unary(_, _) => {}
            _ => {
                return Err("Expected condition".to_string());
            }
        }
        match increment {
            Binary(_, _, _) | Unary(_, _) | Literal(_) => {}
            _ => {
                return Err("Expected increment".to_string());
            }
        }

        Ok(Self::For {
            declaration: declaration.into(),
            condition: condition.into(),
            increment: increment.into(),
            then: then.into(),
        })
    }
}
