use crate::expressions::computable::{Computable, Compute};
use crate::expressions::expression::Expression;

use std::{cell::RefCell, rc::Rc};

/// Multiplies two values
pub struct Mul {
    expression: Expression,
}

impl Mul {
    pub fn new(left: Computable, right: Computable) -> Computable {
        let args = Vec::from([left.clone(), right.clone()]);

        let mul = Self {
            expression: Expression {
                args,
                ..Default::default()
            },
        };

        let mul_rc = Rc::new(RefCell::new(mul));
        left.borrow_mut().add_dependant(mul_rc.clone());
        right.borrow_mut().add_dependant(mul_rc.clone());
        mul_rc
    }
}

impl Compute for Mul {
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
        self.expression.args.iter().fold(1.0f32, |acc, value| {
            return acc * value.borrow_mut().compute();
        })
    }
}
