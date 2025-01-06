use super::EvaluatedExpression;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type ContextRef = Rc<RefCell<Context>>;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Context {
    declarations: HashMap<String, EvaluatedExpression>,

    pub parent: Option<ContextRef>,
    // pub child: Option<ContextRef>,
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
    pub fn merge(first: ContextRef, other: &ContextRef) -> ContextRef {
        let mut root_other = other.borrow().get_root();

        let ctx = Context {
            parent: Some(first.clone()),
            ..Default::default()
        };
        root_other.parent = Some(Rc::new(RefCell::new(ctx)));

        other.clone()
    }
    fn get_root(&self) -> Context {
        if let Some(parent) = &self.parent {
            return parent.borrow().get_root();
        }
        self.clone()
    }

    pub fn get_declaration(&self, variable_name: &str) -> Option<EvaluatedExpression> {
        if let Some(value) = self.declarations.get(variable_name) {
            return Some(value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get_declaration(variable_name);
        }

        None
    }

    pub fn contains_declaration(&self, variable_name: &str) -> bool {
        if self.declarations.contains_key(variable_name) {
            return true;
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().contains_declaration(variable_name);
        }

        false
    }

    pub fn set_declaration(&mut self, variable_name: String, value: EvaluatedExpression) {
        self.declarations.insert(variable_name, value);
    }

    pub fn change_declaration(
        &mut self,
        variable_name: &str,
        value: EvaluatedExpression,
    ) -> Option<EvaluatedExpression> {
        if self.declarations.contains_key(variable_name) {
            self.declarations
                .insert(variable_name.to_string(), value.clone());
            return Some(value);
        } else if let Some(parent) = &mut self.parent {
            return parent.borrow_mut().change_declaration(variable_name, value);
        }
        None
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.declarations.keys().collect::<Vec<_>>())
    }
}
