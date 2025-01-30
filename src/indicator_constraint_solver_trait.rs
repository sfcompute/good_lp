use crate::{constraint::ConstraintReference, Expression, Variable};

/// A trait for solvers that support indicator constraints
pub trait IndicatorConstraintSolver {
    /// Add indicator constraint in the form of lhs <= rhs if `indicator_variable` is true, otherwise the constraint is ignored.
    fn add_indicator_constraint(
        &mut self,
        indicator_variable: Variable,
        lhs: Expression,
        rhs: f64,
    ) -> ConstraintReference;
}
