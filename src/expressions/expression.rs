use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::expressions::computable::{Computable, Compute};

/// Store for lazy expression evaluation. Caches the result.
#[derive(Default)]
pub struct Expression {
    /// Input arguments
    pub args: Vec<Computable>,
    /// Cached evaluation result
    pub cached_value: Option<f32>,
    /// Dependant expressions
    pub dependants: Vec<Weak<RefCell<dyn Compute>>>,
}

impl Compute for Expression {
    fn add_dependant(&mut self, dependant: Computable) {
        self.dependants.push(Rc::downgrade(&dependant));
    }

    fn has_cached_value(&self) -> bool {
        self.cached_value.is_some()
    }

    fn get_cached_value(&self) -> f32 {
        assert!(self.has_cached_value());
        self.cached_value.unwrap()
    }

    fn set_cached_value(&mut self, value: f32) {
        self.cached_value = Some(value);
    }

    fn compute_full(&self) -> f32 {
        unimplemented!("Method should be implemented in superclass")
    }

    fn invalidate_cache(&mut self) {
        self.cached_value = None;

        for dependant in &self.dependants {
            if let Some(dependant) = dependant.upgrade() {
                dependant.borrow_mut().invalidate_cache();
            } else {
                panic!("Failed to get an owning reference to dependant expression")
            }
        }
    }
}
