use crate::{constraint::ConstraintReference, Variable};

/// A trait for solvers that support quadratic constraints
pub trait QuadraticConstraintSolver {
    /// Add quadratic constraint in the form of linear_terms.sum() + quadratic_terms.sum() + constant <= rhs
    fn add_quadratic_constraint(&mut self, terms: &[QuadraticTerm], rhs: f64, is_equality: bool) -> ConstraintReference;
}

/// A term in a quadratic expression
pub enum QuadraticTerm {
    /// A quadratic term in the form of c * a * b
    Quadratic(f64, Variable, Variable),
    /// A linear term in the form of c * a
    Linear(f64, Variable),
    /// A constant term
    Constant(f64),
}
