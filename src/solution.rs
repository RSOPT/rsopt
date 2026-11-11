//! Program solution abstraction.

use crate::{Float, Status};

use nalgebra::SVector;

/// Program solution abstraction.
/// 
/// Program solutions are returned by solvers.  They contain all solution metadata,
/// including program status as well as primal and dual variables, if the problem
/// was solved to optimality.
/// 
/// This solution has `N` primal variables, `E` unconstrained dual variables, and
/// `I` nonnegative dual variables (assuming that the problem was solved to
/// optimality).
pub struct Solution<const N: usize, const E: usize, const I: usize> {
    /// Program status.
    pub status: Status,

    /// Primal variables.
    pub primal: Option<SVector<Float, N>>,

    /// Unconstrained dual variables.
    pub equality_dual: Option<SVector<Float, E>>,

    /// Nonnegative dual variables.
    pub inequality_dual: Option<SVector<Float, I>>,
}
