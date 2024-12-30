use super::Expression;

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
    pub value_type: ValueType,
}
impl std::fmt::Display for EvaluatedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value_type)
    }
}

impl EvaluatedExpression {
    pub fn to_bool(&self) -> bool {
        use ValueType::*;
        match self.value_type {
            BOOL(v) => v,
            NUMBER(v) => v != 0.0,
            STRING(_) => true,
            FUNCTION { .. } => true,
            NIL => false,
        }
    }
    pub fn nil() -> EvaluatedExpression {
        EvaluatedExpression {
            value_type: ValueType::NIL,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    NUMBER(f64),
    STRING(String),
    BOOL(bool),
    FUNCTION {
        name: String,
        params: Vec<String>,
        body: Box<Expression>,
    },
    NIL,
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ValueType::*;
        match self {
            NUMBER(v) => write!(f, "{}", v),
            STRING(v) => write!(f, "{}", v),
            BOOL(v) => write!(f, "{}", v),
            FUNCTION {
                name,
                params,
                body: _,
            } => {
                write!(f, "Function {}({:?})", name, params)
            }
            NIL => write!(f, "nil"),
        }
    }
}

impl Into<EvaluatedExpression> for f64 {
    fn into(self) -> EvaluatedExpression {
        EvaluatedExpression {
            value_type: ValueType::NUMBER(self),
        }
    }
}

impl Into<EvaluatedExpression> for bool {
    fn into(self) -> EvaluatedExpression {
        EvaluatedExpression {
            value_type: ValueType::BOOL(self),
        }
    }
}

impl Into<EvaluatedExpression> for String {
    fn into(self) -> EvaluatedExpression {
        EvaluatedExpression {
            value_type: ValueType::STRING(self),
        }
    }
}
impl Into<EvaluatedExpression> for &str {
    fn into(self) -> EvaluatedExpression {
        EvaluatedExpression {
            value_type: ValueType::STRING(self.to_string()),
        }
    }
}
