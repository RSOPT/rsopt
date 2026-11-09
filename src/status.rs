//! Program status enumeration.

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
/// Program status enumeration.
/// 
/// Convex programs fall into three possible categories:
/// (1) __Optimal__.  The program has been solved to global optimality.
/// (2) __Unbounded__.  The objective can be improved indefinitely.
/// (3) __Infeasible__.  There are no feasible points.
pub enum Status {
    /// Optimal status.
    Optimal,

    /// Unbounded status.
    Unbounded,

    /// Infeasible status.
    Infeasible,
}
