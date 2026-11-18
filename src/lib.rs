//! Main library for RSOPT.
//! 
//! RSOPT is a simple, elegant convex optimization solver written in
//! pure `#![no_std]` Rust.  Rather than serve as a general-purpose
//! convex optimizer, RSOPT is actually a _family_ of parameterized
//! convex optimization programs.  Program sizes are determined at
//! compile time, but the parameters may be provided at runtime.
//! At compile time, RSOPT dispatches the program to the pertinent
//! solver.
//! 
//! RSOPT is different from many existing convex optimization solvers
//! and systems.  Unlike CVX, RSOPT is not a frontend system and
//! does not verify programs against grammars like DCP.  Unlike
//! SNOPT or IPOPT, RSOPT is not a general-purpose nonlinear optimizer.
//! As the RSOPT project grows, we intend to grow the family of
//! parameterized convex programs so that RSOPT can be increasingly
//! useful to the research community.

#![no_std]
#![deny(warnings)]
#![deny(missing_docs)]

mod classes;
mod config;
mod program;
mod solution;
mod status;

pub use classes::{
    lp,
    qp,
};
pub use config::SolverConfig;
pub use program::MathematicalProgram;
pub use solution::Solution;
pub use status::Status;

/// Floating-point precision used for the solver.
pub type Float = f64;
