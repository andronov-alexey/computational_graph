use crate::expressions::computable::{Computable, Compute};
use crate::expressions::expression::Expression;

use std::{cell::RefCell, rc::Rc};

/// Adds two values
pub struct Add {
    expression: Expression,
}

impl Add {
    pub fn new(left: Computable, right: Computable) -> Computable {
        let args = Vec::from([left.clone(), right.clone()]);

        let add = Self {
            expression: Expression {
                args,
                ..Default::default()
            },
        };

        let self_rc = Rc::new(RefCell::new(add));
        left.borrow_mut().add_dependant(self_rc.clone());
        right.borrow_mut().add_dependant(self_rc.clone());
        self_rc
    }
}

impl Compute for Add {
    fn add_dependant(&mut self, dependant: Computable) {
        self.expression.add_dependant(dependant);
    }

    fn has_cached_value(&self) -> bool {
        self.expression.has_cached_value()
    }

    fn get_cached_value(&self) -> f32 {
        self.expression.get_cached_value()
    }

    fn set_cached_value(&mut self, value: f32) {
        self.expression.set_cached_value(value);
    }

    fn invalidate_cache(&mut self) {
        self.expression.invalidate_cache()
    }

    fn compute_full(&self) -> f32 {
        self.expression.args.iter().fold(0.0f32, |acc, value| {
            return acc + value.borrow_mut().compute();
        })
    }
}
