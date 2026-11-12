//! RSOPT
//! Copyright (c) 2026- The RSOPT Project
//! 
//! Unconstrained QP tests.

use nalgebra::{
    matrix,
    vector,
};

use rsopt::{
    MathematicalProgram,
    SolverConfig,
    Status,
    qp::UnconstrainedQuadraticProgram,
};

#[test]
fn solve_trivial_2d_qp() {
    // Specify parameters
    let a = matrix![
        1.0, 0.0;
        0.0, 1.0;
    ];
    let b = vector![ 0.0, 0.0 ];

    // Construct and solve QP
    let qp = UnconstrainedQuadraticProgram::<2>::new(
        a, b
    );    
    let solution = qp.solve(SolverConfig::default());

    // Check QP result
    assert_eq!(solution.status, Status::Optimal);
    assert_eq!(
        solution.primal,
        Some (vector![ 0.0, 0.0 ]),
    );
    assert!(solution.equality_dual.is_none());
    assert!(solution.inequality_dual.is_none());
}

#[test]
fn solve_trivial_3d_qp() {
    // Specify parameters
    let a = matrix![
        1.0, 0.0, 0.0;
        0.0, 1.0, 0.0;
        0.0, 0.0, 1.0;
    ];
    let b = vector![ 1.0, -2.0, 3.0 ];

    // Construct and solve QP
    let qp = UnconstrainedQuadraticProgram::<3>::new(
        a, b
    );    
    let solution = qp.solve(SolverConfig::default());

    // Check QP result
    assert_eq!(solution.status, Status::Optimal);
    assert_eq!(
        solution.primal,
        Some (vector![ -1.0, 2.0, -3.0 ]),
    );
    assert!(solution.equality_dual.is_none());
    assert!(solution.inequality_dual.is_none());
}

#[test]
fn solve_2d_qp() {
    // Specify parameters
    let a = matrix![
        2.0, -1.0;
        -1.0, 2.0;
    ];
    let b = vector![ -4.0, 2.0 ];

    // Construct and solve QP
    let qp = UnconstrainedQuadraticProgram::<2>::new(
        a, b
    );    
    let solution = qp.solve(SolverConfig::default());

    // Check QP result
    assert_eq!(solution.status, Status::Optimal);
    assert_eq!(
        solution.primal,
        Some (vector![ 2.0, 0.0 ]),
    );
    assert!(solution.equality_dual.is_none());
    assert!(solution.inequality_dual.is_none());
}
