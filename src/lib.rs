//! RSOPT
//! Copyright (c) 2026- The RSOPT Project
//!
//! Main library for RSOPT.

#![no_std]
#![deny(warnings)]
#![deny(missing_docs)]

mod classes;
mod config;
mod program;
mod solution;
mod status;

pub use classes::{
    qp,
};
pub use config::SolverConfig;
pub use program::MathematicalProgram;
pub use solution::Solution;
pub use status::Status;

/// Floating-point precision used for the solver.
pub type Float = f64;
