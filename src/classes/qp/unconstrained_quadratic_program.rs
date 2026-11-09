//! Unconstrained quadratic program.

use nalgebra::{SMatrix, SVector};

use crate::{Float, MathematicalProgram, Solution, SolverConfig, Status};

/// Unconstrained quadratic program.
/// 
/// This implementation of unconstrained QP uses the conjugate gradient method.
/// For more information on the conjugate gradient method, refer to the section
/// titled "Unconstrained Quadratic Program" in the technical report.
pub struct UnconstrainedQuadraticProgram<const N: usize> {
    #[allow(dead_code)]
    /// PSD matrix `A`.
    matrix: SMatrix<Float, N, N>,

    #[allow(dead_code)]
    /// Vector `b`.
    vector: SVector<Float, N>,
}

impl<const N: usize> UnconstrainedQuadraticProgram<N> {
    /// Construct this QP from a matrix and a vector.
    pub fn new(matrix: SMatrix<Float, N, N>, vector: SVector<Float, N>) -> Self {
        Self {
            matrix,
            vector,
        }
    }
}

impl<const N: usize> MathematicalProgram<N, 0> for UnconstrainedQuadraticProgram<N> {
    fn solve(&self, config: SolverConfig) -> Solution<N, 0> {
        // Solution
        let mut x: SVector<Float, N> = SVector::zeros();

        // Initial search direction and gradient
        let mut p: SVector<Float, N> = self.vector;
        let mut g: SVector<Float, N> = self.vector;
        if g.max().abs() >= config.primal_gradient_tolerance {
            // Step size
            let mut a: Float = -g.dot(&p) / p.dot(&(self.matrix * p));
            
            // Update
            x += a * p;

            for _ in 1..N {
                // Calculate gradient direction and step direction
                let ap = self.matrix * p;
                g = self.matrix * x + self.vector;
                if g.max().abs() < config.primal_gradient_tolerance {
                    break;
                }
                p = g - (g.dot(&ap) / p.dot(&ap)) * p;

                // Calculate step size
                let ap = self.matrix * p;
                a = -g.dot(&p) / p.dot(&ap);

                // Update
                x += a * p;
            }
        }

        Solution {
            status: Status::Optimal,
            primal: Some (x),
            dual: None,
        }
    }
}
