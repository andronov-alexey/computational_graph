use crate::expressions::computable::{Computable, Compute};
use crate::expressions::expression::Expression;

use std::{cell::RefCell, rc::Rc};

/// Raises a value to the power of exp
pub struct Pow {
    expression: Expression,
    pub exp: f32,
}

impl Pow {
    pub fn new(arg: Computable, exp: f32) -> Computable {
        let args = Vec::from([arg.clone()]);

        let pow = Self {
            expression: Expression {
                args,
                ..Default::default()
            },
            exp,
        };

        let pow_rc = Rc::new(RefCell::new(pow));
        arg.borrow_mut().add_dependant(pow_rc.clone());
        pow_rc
    }
}

impl Compute for Pow {
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
        assert_eq!(self.expression.args.len(), 1);
        self.expression
            .args
            .get(0)
            .unwrap()
            .borrow_mut()
            .compute()
            .powf(self.exp)
    }
}
