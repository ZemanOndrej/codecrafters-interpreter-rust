#[derive(Debug, Clone, PartialEq)]
pub enum EvaluatedExpressionResult {
    FunctionReturn(EvaluatedExpression),
    Value(EvaluatedExpression),
}
impl From<EvaluatedExpression> for EvaluatedExpressionResult {
    fn from(value: EvaluatedExpression) -> Self {
        EvaluatedExpressionResult::Value(value)
    }
}

impl EvaluatedExpressionResult {
    pub fn assert_value(self) -> Result<EvaluatedExpression, String> {
        match self {
            EvaluatedExpressionResult::Value(value) => Ok(value),
            _ => Err("Expected value not return.".to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluatedExpression {
    pub value: String,
    pub value_type: ValueType,
}

impl EvaluatedExpression {
    pub fn to_bool(&self) -> bool {
        match self.value_type {
            ValueType::BOOL => self.value.parse().unwrap(),
            ValueType::NUMBER => self.value != "0",
            ValueType::STRING => true,
            ValueType::NIL => false,
        }
    }
    pub fn nil() -> EvaluatedExpression {
        EvaluatedExpression {
            value: "nil".to_string(),
            value_type: ValueType::NIL,
        }
    }
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
