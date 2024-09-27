pub struct EvaluatedExpression {
    pub value: String,
    pub value_type: ValueType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    NUMBER,
    STRING,
    BOOL,
    NIL,
}
impl Into<EvaluatedExpression> for f64 {
    fn into(self) -> EvaluatedExpression {
        EvaluatedExpression {
            value: self.to_string(),
            value_type: ValueType::NUMBER,
        }
    }
}

impl Into<EvaluatedExpression> for bool {
    fn into(self) -> EvaluatedExpression {
        EvaluatedExpression {
            value: self.to_string(),
            value_type: ValueType::BOOL,
        }
    }
}

impl Into<EvaluatedExpression> for String {
    fn into(self) -> EvaluatedExpression {
        EvaluatedExpression {
            value: self,
            value_type: ValueType::STRING,
        }
    }
}
impl Into<EvaluatedExpression> for &str {
    fn into(self) -> EvaluatedExpression {
        EvaluatedExpression {
            value: self.to_string(),
            value_type: ValueType::STRING,
        }
    }
}
