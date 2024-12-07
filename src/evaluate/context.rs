use super::EvaluatedExpression;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type ContextRef = Rc<RefCell<Context>>;

#[derive(Debug, Clone, Default)]
pub struct Context {
    variables: HashMap<String, EvaluatedExpression>,
    pub parent: Option<ContextRef>,
    pub child: Option<ContextRef>,
}

impl Context {
    pub fn new_root() -> ContextRef {
        Rc::new(RefCell::new(Context::default()))
    }
    pub fn new(parent: ContextRef) -> ContextRef {
        let ctx = Context {
            parent: Some(parent),
            ..Default::default()
        };
        Rc::new(RefCell::new(ctx))
    }

    pub fn get_variable(&self, variable_name: &str) -> Option<EvaluatedExpression> {
        if let Some(value) = self.variables.get(variable_name) {
            return Some(value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get_variable(variable_name);
        }

        None
    }

    pub fn contains_variable(&self, variable_name: &str) -> bool {
        if self.variables.contains_key(variable_name) {
            return true;
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().contains_variable(variable_name);
        }

        false
    }
    pub fn set_variable(&mut self, variable_name: String, value: EvaluatedExpression) {
        self.variables.insert(variable_name, value);
    }

    pub fn change_variable(
        &mut self,
        variable_name: &str,
        value: EvaluatedExpression,
    ) -> Option<EvaluatedExpression> {
        if self.variables.contains_key(variable_name) {
            self.variables
                .insert(variable_name.to_string(), value.clone());
            return Some(value);
        } else {
            if let Some(parent) = &mut self.parent {
                return parent.borrow_mut().change_variable(variable_name, value);
            }
        }
        return None;
    }
}
