use crate::expressions::computable::{Computable, Compute};

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// Named variable
#[derive(Clone, Default)]
pub struct Input {
    /// Variable name
    pub name: &'static str,
    /// Variable value
    pub value: f32,
    /// List of expressions dependant on the current input value
    dependants: Vec<Weak<RefCell<dyn Compute>>>,
}

impl Input {
    /// Create named input variable
    pub fn new(name: &'static str) -> Rc<RefCell<Input>> {
        let input = Self {
            name,
            ..Default::default()
        };

        Rc::new(RefCell::new(input))
    }

    /// Change input variable value
    pub fn set(&mut self, value: f32) {
        self.value = value;
        self.invalidate_cache();
    }
}

impl Compute for Input {
    fn add_dependant(&mut self, dependant: Computable) {
        self.dependants.push(Rc::downgrade(&dependant))
    }

    fn has_cached_value(&self) -> bool {
        true
    }

    fn get_cached_value(&self) -> f32 {
        self.value
    }

    fn set_cached_value(&mut self, value: f32) {
        assert_eq!(self.value, value);
    }

    fn invalidate_cache(&mut self) {
        for dependant in &self.dependants {
            if let Some(dependant) = dependant.upgrade() {
                dependant.borrow_mut().invalidate_cache();
            } else {
                panic!("Failed to get an owning reference to dependant expression")
            }
        }
    }

    fn compute_full(&self) -> f32 {
        self.value
    }
}
