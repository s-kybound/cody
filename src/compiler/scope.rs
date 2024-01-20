use std::{collections::HashMap, cell::RefCell};

use inkwell::values::PointerValue;

impl<'a> Scope<'a> {
    pub fn new(parent: Option<Box<Scope<'a>>>) -> Scope<'a> {
        Scope {
            parent,
            variables: RefCell::new(HashMap::new())
        }
    }

    pub fn add_variable(&self, name: String, value: PointerValue<'a>) {
        let mut vars = self.variables.borrow_mut();
        vars.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<PointerValue<'a>> {
        let vars = self.variables.borrow();
        match vars.get(name) {
            Some(v) => Some(*v),
            None => match &self.parent {
                Some(p) => p.get_variable(name),
                None => None
            }
        }
    }
}

pub struct Scope<'a> {
    pub parent: Option<Box<Scope<'a>>>,
    pub variables: RefCell<HashMap<String, PointerValue<'a>>>
}