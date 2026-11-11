//! Linear program.

use nalgebra::{SMatrix, SVector};

use crate::{Float, MathematicalProgram, Solution, SolverConfig, Status};

/// Linear program.
/// 
/// The canonical linear program has equality constraints, inequality constraints,
/// and requires all decision variables to be nonnegative.  Reference "Linear Program"
/// in the RSOPT technical report for details.
/// 
/// This implementation uses the _primal-dual interior point method_ (PDIPM) to solve
/// linear programs in polynomial time.  We opt for this over the simplex method for
/// algorithmic simplicity in addition to improved convergence guarantees.  It is
/// important to emphasize that, in practice, the simplex method demonstrates excellent
/// performance despite its exponential-time convergence in the worst case.  We also
/// acknowledge that the simplex algorithm, unlike PDIPM, converges to exact solutions
/// whereas PDIPM only converges within numerical tolerances, as the solution travels
/// along a central path in the interior of the feasible set.
/// 
/// Future work on RSOPT may implement a simplex method solver for LPs; however, this
/// is dependent on downstream requirements and currently such work is not planned.
pub struct LinearProgram<const N: usize, const E: usize, const I: usize> {
    /// Vector `c` defining the objective.
    objective_vector: SVector<Float, N>,

    /// Matrix `E` for affine equality constraint.
    equality_matrix: SMatrix<Float, E, N>,

    /// Matrix `A` for affine inequality constraint.
    inequality_matrix: SMatrix<Float, I, N>,

    /// Vector `d` for affine equality constraint.
    equality_vector: SVector<Float, E>,

    /// Vector `b` for affine inequality constraint.
    inequality_vector: SVector<Float, I>,
}

impl<const N: usize, const E: usize, const I: usize> LinearProgram<N, E, I> {
    /// Construct this linear program.
    pub fn new(
        objective_vector: SVector<Float, N>,
        equality_matrix: SMatrix<Float, E, N>,
        equality_vector: SVector<Float, E>,
        inequality_matrix: SMatrix<Float, I, N>,
        inequality_vector: SVector<Float, I>,
    ) -> Self {
        Self {
            objective_vector,
            equality_matrix,
            inequality_matrix,
            equality_vector,
            inequality_vector,
        }
    }
}

impl<const N: usize, const E: usize, const I: usize> MathematicalProgram<N, E, I> for LinearProgram<N, E, I> {
    fn solve(&self, config: SolverConfig) -> Solution<N, E, I> {
        // Bring useful things into the current namespace
        let e = self.equality_matrix;
        let a = self.inequality_matrix;
        let d = self.equality_vector;
        let b = self.inequality_vector;
        let c = self.objective_vector;

        // Barrier parameter
        let mut mu = 10.0;

        // Vector of ones
        let mut ones = SVector::<Float, I>::zeros();
        ones.fill(1.0);

        // Construct current guess for primals and duals
        // TODO improve these guesses!
        let mut x = SVector::<Float, N>::zeros();
        let mut lambda = SVector::<Float, I>::zeros();
        let mut eta = SVector::<Float, E>::zeros();
        x.fill(1.0);
        lambda.fill(1.0);

        for _ in 0..config.major_iteration_limit {
            // Inequality residual
            let s = a * x - b;

            // Compute divisions
            let s_inverse: SVector<Float, I> = ones.component_div(&s);
            let lambda_over_s = lambda.component_mul(&s_inverse);

            // Construct KKT residuals
            let p: SVector<Float, N> = -c - a.transpose() * lambda;
            let q: SVector<Float, E> = d - e * x;
            let r: SVector<Float, I> = mu * ones - lambda.component_mul(&s);

            // Intermediate "helper" matrix
            // TODO fix unwrap
            let z: SMatrix<Float, N, N> = a.transpose() * SMatrix::from_diagonal(&lambda_over_s) * a;
            println!("z = {:#?}", z);
            let reg = config.regularization * SMatrix::<Float, N, N>::identity();
            let z_inverse = (z + reg).try_inverse().unwrap();

            // Intermediate "helper" vector
            let y: SVector<Float, N> = a.transpose() * SMatrix::from_diagonal(&s_inverse) * r;

            // Update dual
            // TODO fix unwrap
            let e_zinv = e * z_inverse;
            eta = (e_zinv * e.transpose()).try_inverse().unwrap() * (q + e_zinv * (p - y));

            // Calculate primal update direction
            let dx = z_inverse * (e.transpose() * eta + y - p);

            // Calculate dual update direction
            let dlambda = SMatrix::from_diagonal(&s_inverse) * (r - lambda.component_mul(&(a * x)));

            println!("dx = {:#?}", dx);
            println!("dlambda = {:#?}", dlambda);

            // Calculate update sizes
            let tau = config.boundary_coefficient;
            let primal_alphas = -tau * x.component_div(&dx);
            let dual_alphas = -tau * lambda.component_div(&dlambda);
            let mut primal_alpha = 1.0;
            for a in &primal_alphas {
                if *a > 0.0 && *a < primal_alpha {
                    primal_alpha = *a;
                }
            }
            let mut dual_alpha = 1.0;
            for a in &dual_alphas {
                if *a > 0.0 && *a < dual_alpha {
                    dual_alpha = *a;
                }
            }
            println!("primal a = {:#?}", primal_alphas);
            println!("dual a = {:#?}", dual_alphas);

            println!("primal a = {:.6}", primal_alpha);
            println!("dual a = {:.6}", dual_alpha);

            x += primal_alpha * dx;
            lambda += dual_alpha * dlambda;

            println!("x = {:#?}", x);
            println!("lambda = {:#?}", lambda);

            // Update barrier parameter
            let duality_gap = (a * x - b).dot(&lambda) / (N as Float);
            mu = 0.5 * duality_gap;
            println!("s = {:#?}", a * x - b);
            println!("mu = {:.6}", mu);
        }

        Solution {
            status: Status::Infeasible,
            primal: Some (x),
            equality_dual: Some (eta),
            inequality_dual: Some (lambda),
        }
    }
}
