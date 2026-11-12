//! RSOPT
//! Copyright (c) 2026- The RSOPT Project
//! 
//! LP tests.

use nalgebra::{
    SMatrix,
    SVector,
    matrix,
    vector,
};

use rsopt::{
    Float,
    MathematicalProgram,
    SolverConfig,
    Status,
    lp::LinearProgram,
};

const L1_THRESHOLD: Float = 1e-6;

/// Determine if two vectors are within a fixed L1 distance.
fn appxeq<const N: usize>(x: SVector<Float, N>, y: SVector<Float, N>) -> bool {
    (x - y).abs().max() < L1_THRESHOLD
}

#[test]
fn solve_trivial_lp() {
    // Construct LP
    let lp = LinearProgram::<2, 0, 0>::new(
        vector![1.0, 1.0],
        SMatrix::<Float, 0, 2>::zeros(),
        SVector::<Float, 0>::zeros(),
        SMatrix::<Float, 0, 2>::zeros(),
        SVector::<Float, 0>::zeros(),
    );

    // Define configuration
    let mut config = SolverConfig::default();
    config.major_iteration_limit = 25;

    // Solve
    let result = lp.solve(config);

    assert_eq!(result.status, Status::Optimal);
    println!("{:#?}", result.primal);
    assert!(result.primal.is_some());
    assert!(appxeq(result.primal.unwrap(), vector![0.0, 0.0]));
}

#[test]
fn solve_2d_lp() {
    // Construct LP
    let lp = LinearProgram::<2, 0, 2>::new(
        vector![1.0, 1.0],
        SMatrix::<Float, 0, 2>::zeros(),
        SVector::<Float, 0>::zeros(),
        matrix![1.0, 2.0; 2.0, 1.0],
        vector![2.0, 2.0],
    );

    // Define configuration
    let mut config = SolverConfig::default();
    config.major_iteration_limit = 25;

    // Solve
    let result = lp.solve(config);

    assert_eq!(result.status, Status::Optimal);
    assert!(result.primal.is_some());
    assert!(appxeq(result.primal.unwrap(), vector![2.0/3.0, 2.0/3.0]));
}

#[test]
fn solve_2d_equality_lp() {
    // Construct LP
    let lp = LinearProgram::<2, 1, 2>::new(
        vector![1.0, 1.0],
        matrix![1.0, 0.0],
        vector![1.5],
        matrix![1.0, 2.0; 2.0, 1.0],
        vector![2.0, 2.0],
    );

    // Define configuration
    let mut config = SolverConfig::default();
    config.major_iteration_limit = 25;

    // Solve
    let result = lp.solve(config);

    assert_eq!(result.status, Status::Optimal);
    assert!(result.primal.is_some());
    assert!(appxeq(result.primal.unwrap(), vector![1.5, 0.25]));
}

#[test]
fn solve_2d_feasibility_lp() {
    // Construct LP
    let lp = LinearProgram::<2, 2, 0>::new(
        vector![1.0, 1.0],
        matrix![1.0, 1.0; 1.0, -1.0],
        vector![2.0, 0.0],
        SMatrix::<Float, 0, 2>::zeros(),
        SVector::<Float, 0>::zeros(),
    );

    // Define configuration
    let mut config = SolverConfig::default();
    config.major_iteration_limit = 25;

    // Solve
    let result = lp.solve(config);

    assert_eq!(result.status, Status::Optimal);
    assert!(result.primal.is_some());
    assert!(appxeq(result.primal.unwrap(), vector![1.0, 1.0]));
}

#[test]
fn solve_2d_infeasible_lp() {
    // Construct LP
    let lp = LinearProgram::<2, 2, 0>::new(
        vector![1.0, 1.0],
        matrix![1.0, 1.0; 1.0, 1.0],
        vector![2.0, 4.0],
        SMatrix::<Float, 0, 2>::zeros(),
        SVector::<Float, 0>::zeros(),
    );

    // Define configuration
    let mut config = SolverConfig::default();
    config.major_iteration_limit = 25;

    // Solve
    let result = lp.solve(config);

    assert_eq!(result.status, Status::Infeasible);
}

#[test]
fn solve_2d_barely_infeasible_lp() {
    // Construct LP
    let lp = LinearProgram::<2, 2, 0>::new(
        vector![1.0, 1.0],
        matrix![1.0, 1.0; 1.0, 1.0],
        vector![2.0, 2.0 + 1e-9],
        SMatrix::<Float, 0, 2>::zeros(),
        SVector::<Float, 0>::zeros(),
    );

    // Define configuration
    let mut config = SolverConfig::default();
    config.major_iteration_limit = 25;

    // Solve
    let result = lp.solve(config);

    assert_eq!(result.status, Status::Infeasible);
}

#[test]
fn solve_2d_unbounded_lp() {
    // Construct LP
    let lp = LinearProgram::<2, 0, 2>::new(
        vector![-1.0, -1.0],
        SMatrix::<Float, 0, 2>::zeros(),
        SVector::<Float, 0>::zeros(),
        matrix![1.0, 2.0; 2.0, 1.0],
        vector![2.0, 2.0],
    );

    // Define configuration
    let mut config = SolverConfig::default();
    config.major_iteration_limit = 25;

    // Solve
    let result = lp.solve(config);

    assert_eq!(result.status, Status::Unbounded);
}
