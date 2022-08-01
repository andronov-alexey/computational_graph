//! # Computational graph
//!
//! `computational_graph` crate provides the ability to create a graph where each node is a lazy expression.<br>
//! Graph evaluates the expression on demand and caches the result.<br>
//! When the input changes only minimally necessary caches cleared.<br>

pub mod expressions;
pub mod ops;
pub mod utils;

mod tests {
    use std::f32::consts::PI;

    use crate::expressions::computable::Compute;
    use crate::expressions::input::Input;
    use crate::ops::add::Add;
    use crate::ops::mul::Mul;
    use crate::ops::pow::Pow;
    use crate::ops::sin::Sin;
    use crate::utils::round;

    #[test]
    fn example_test() {
        // y = x1 + x2 * sin(x2 + pow(x3, 3))
        // x1, x2, x3 are input nodes of the computational graph:
        let x1 = Input::new("x1");
        let x2 = Input::new("x2");
        let x3 = Input::new("x3");
        // graph variable is the output node of the graph:
        let graph = Add::new(
            x1.clone(),
            Mul::new(
                x2.clone(),
                Sin::new(Add::new(x2.clone(), Pow::new(x3.clone(), 3f32))),
            ),
        );
        x1.borrow_mut().set(1f32);
        x2.borrow_mut().set(2f32);
        x3.borrow_mut().set(3f32);
        let mut result = graph.borrow_mut().compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), -0.32727);
        x1.borrow_mut().set(2f32);
        x2.borrow_mut().set(3f32);
        x3.borrow_mut().set(4f32);
        result = graph.borrow_mut().compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), -0.56656);
    }

    #[test]
    fn example_destructured_ops_test() {
        // y = x1 + x2 * sin(x2 + pow(x3, 3))
        // x1, x2, x3 are input nodes of the computational graph:
        let x1 = Input::new("x1");
        let x2 = Input::new("x2");
        let x3 = Input::new("x3");

        let pow_op = Pow::new(x3.clone(), 3f32);
        let add_op = Add::new(x2.clone(), pow_op);
        let sin_op = Sin::new(add_op);
        let mul_op = Mul::new(x2.clone(), sin_op);
        // graph variable is the output node of the graph:
        let graph = Add::new(x1.clone(), mul_op);
        x1.borrow_mut().set(1f32);
        x2.borrow_mut().set(2f32);
        x3.borrow_mut().set(3f32);
        let mut result = graph.borrow_mut().compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), -0.32727);
        x1.borrow_mut().set(2f32);
        x2.borrow_mut().set(3f32);
        x3.borrow_mut().set(4f32);
        result = graph.borrow_mut().compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), -0.56656);
    }

    #[test]
    fn example_destructured_ops_cache_test() {
        // y = x1 + x2 * sin(x2 + pow(x3, 3))
        // x1, x2, x3 are input nodes of the computational graph:
        let x1 = Input::new("x1");
        let x2 = Input::new("x2");
        let x3 = Input::new("x3");

        let pow_op = Pow::new(x3.clone(), 3f32);
        let add_op = Add::new(x2.clone(), pow_op.clone());
        let sin_op = Sin::new(add_op.clone());
        let mul_op = Mul::new(x2.clone(), sin_op.clone());
        // graph variable is the output node of the graph:
        let graph = Add::new(x1.clone(), mul_op.clone());
        assert!(!pow_op.clone().borrow().has_cached_value());
        assert!(!add_op.clone().borrow().has_cached_value());
        assert!(!sin_op.clone().borrow().has_cached_value());
        assert!(!mul_op.clone().borrow().has_cached_value());
        assert!(!graph.clone().borrow().has_cached_value());
        x1.borrow_mut().set(1f32);
        x2.borrow_mut().set(2f32);
        x3.borrow_mut().set(3f32);
        let mut result = graph.borrow_mut().compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), -0.32727);
        assert!(pow_op.clone().borrow().has_cached_value());
        assert!(add_op.clone().borrow().has_cached_value());
        assert!(sin_op.clone().borrow().has_cached_value());
        assert!(mul_op.clone().borrow().has_cached_value());
        assert!(graph.clone().borrow().has_cached_value());

        x1.borrow_mut().set(2f32);
        // need to recalculate only one sum since all other calculations are cached
        assert!(pow_op.clone().borrow().has_cached_value());
        assert!(add_op.clone().borrow().has_cached_value());
        assert!(sin_op.clone().borrow().has_cached_value());
        assert!(mul_op.clone().borrow().has_cached_value());
        assert!(!graph.clone().borrow().has_cached_value());

        result = graph.borrow_mut().compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), 0.67273);
        assert!(pow_op.clone().borrow().has_cached_value());
        assert!(add_op.clone().borrow().has_cached_value());
        assert!(sin_op.clone().borrow().has_cached_value());
        assert!(mul_op.clone().borrow().has_cached_value());
        assert!(graph.clone().borrow().has_cached_value());

        x2.borrow_mut().set(6f32);
        // need to recalculate almost all values and only one is cached
        assert!(pow_op.clone().borrow().has_cached_value());
        assert!(!add_op.clone().borrow().has_cached_value());
        assert!(!sin_op.clone().borrow().has_cached_value());
        assert!(!mul_op.clone().borrow().has_cached_value());
        assert!(!graph.clone().borrow().has_cached_value());
        result = graph.borrow_mut().compute();
        result = round(result, 5);
        println!("Graph output = {}", result);
        assert_eq!(round(result, 5), 7.99947);
    }

    #[test]
    fn add_test() {
        let x1 = Input::new("x1");
        let x2 = Input::new("x2");
        x1.borrow_mut().set(1.0);
        x2.borrow_mut().set(2.34567);
        let add = Add::new(x1, x2);
        let add = add.borrow_mut().compute();
        assert_eq!(round(add, 5), 3.34567);
    }

    #[test]
    fn mul_test() {
        let x1 = Input::new("x1");
        let x2 = Input::new("x2");
        x1.borrow_mut().set(-2.0);
        x2.borrow_mut().set(3.56789);
        let mul = Mul::new(x1, x2);
        let mul = mul.borrow_mut().compute();
        assert_eq!(round(mul, 5), -7.13578);
    }

    #[test]
    fn pow_test() {
        let x1 = Input::new("x1");
        x1.borrow_mut().set(2.0);
        let pow = Pow::new(x1, 3.0);
        let pow = pow.borrow_mut().compute();
        assert_eq!(round(pow, 5), 8.0);
    }

    #[test]
    fn sin_test() {
        let x1 = Input::new("x1");
        x1.borrow_mut().set(PI / 2.0);
        let sin = Sin::new(x1);
        let sin = sin.borrow_mut().compute();
        assert_eq!(round(sin, 5), 1.0);
    }
}
