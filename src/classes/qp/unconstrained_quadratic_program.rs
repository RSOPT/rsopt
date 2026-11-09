//! RSOPT
//! Copyright (c) 2026- The RSOPT Project
//! 
//! Unconstrained quadratic program.

use nalgebra::{SMatrix, SVector};

use crate::{Float, MathematicalProgram, Solution, SolverConfig, Status};

/// Unconstrained quadratic program.
/// 
/// This implementation of unconstrained QP uses the conjugate gradient method.
pub struct UnconstrainedQuadraticProgram<const N: usize> {
    #[allow(dead_code)]
    /// PSD matrix `A`.
    matrix: SMatrix<Float, N, N>,

    #[allow(dead_code)]
    /// Vector `b`.
    vector: SVector<Float, N>,
}

impl<const N: usize> MathematicalProgram<N, 0> for UnconstrainedQuadraticProgram<N> {
    fn solve(&self, _config: SolverConfig) -> Solution<N, 0> {
        // Solution
        let mut x: SVector<Float, N> = SVector::zeros();

        // Initial search direction and gradient
        let mut p: SVector<Float, N> = self.vector;
        let mut g: SVector<Float, N> = self.vector;

        // Step size
        let mut a: Float = -g.dot(&p) / p.dot(&(self.matrix * p));
        
        // Update
        x += a * p;

        for _ in 1..N {
            let ap = self.matrix * p;
            g = self.matrix * x + self.vector;
            p = g - (g.dot(&ap) / p.dot(&ap)) * p;
            a = -g.dot(&p) / p.dot(&ap);
            x += a * p;
        }

        Solution {
            status: Status::Optimal,
            primal: Some (x),
            dual: None,
        }
    }
}
