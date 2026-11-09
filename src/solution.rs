//! RSOPT
//! Copyright (c) 2026- The RSOPT Project
//! 
//! Program solution abstraction.

use crate::{Float, Status};

use nalgebra::SVector;

/// Program solution abstraction.
/// 
/// Program solutions are returned by solvers.  They contain all solution metadata,
/// including program status as well as primal and dual variables, if the problem
/// was solved to optimality.
/// 
/// This solution has `N` primal variables and `K` dual variables (assuming that the
/// problem was solved to optimality).
pub struct Solution<const N: usize, const K: usize> {
    /// Program status.
    pub status: Status,

    /// Primal variables.
    pub primal: Option<SVector<Float, N>>,

    /// Dual variables.
    pub dual: Option<SVector<Float, K>>,
}
