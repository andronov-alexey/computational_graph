use crate::expressions::computable::{Computable, Compute};
use crate::expressions::expression::Expression;

use std::{cell::RefCell, rc::Rc};

/// Computes the sine of a number (in radians)
pub struct Sin {
    expression: Expression,
}

impl Sin {
    pub fn new(arg: Computable) -> Computable {
        let args = Vec::from([arg.clone()]);

        let sin = Self {
            expression: Expression {
                args,
                ..Default::default()
            },
        };

        let sin_rc = Rc::new(RefCell::new(sin));
        arg.borrow_mut().add_dependant(sin_rc.clone());
        sin_rc
    }
}

impl Compute for Sin {
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
            .sin()
    }
}
