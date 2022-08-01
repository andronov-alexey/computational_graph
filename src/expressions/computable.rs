use std::{cell::RefCell, rc::Rc};

/// Reference-counted pointer to something that can be evaluated
pub type Computable = Rc<RefCell<dyn Compute>>;

/// Interface for something that can be computed on demand
pub trait Compute {
    /// Evaluates the expression.<br>
    /// Returns cached value if any, otherwise evaluates only necessary dependant expressions
    fn compute(&mut self) -> f32 {
        if self.has_cached_value() {
            return self.get_cached_value();
        } else {
            let result = self.compute_full();
            self.set_cached_value(result);
            result
        }
    }

    /// Marks `dependant`'s cache dependant on current object's cache (`self`)
    fn add_dependant(&mut self, dependant: Computable);
    /// Check whether the expression is already evaluated
    fn has_cached_value(&self) -> bool;
    /// Get cached expression evaluation result
    fn get_cached_value(&self) -> f32;
    /// Store expression evaluation result in cache
    fn set_cached_value(&mut self, value: f32);
    /// Invalidate inner cache and caches of the all dependant expressions added via `add_dependant` function
    fn invalidate_cache(&mut self);

    /// Evaluates the expression when no cached value is available
    fn compute_full(&self) -> f32;
}
