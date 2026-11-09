# RSOPT

<p align="center">
<img src="https://github.com/RSOPT/rsopt/blob/master/rsopt.png" alt="The stylized letters 'RSOPT' on a dark background." width="200" height="200">
</p>

![Rust](https://github.com/RSOPT/rsopt/actions/workflows/rust.yml/badge.svg)

A simple, elegant convex optimization solver.  Powered by the Rust Programming Language.

## Using RSOPT

Currently, RSOPT can only be called from Rust.  We plan to develop Python bindings and a standalone binary for the optimizer in the future.  As of yet, no development schedule is available.  This repository will be updated to reflect these changes as they are available.

### Calling RSOPT from Rust

All optimization programs in RSOPT exist under the `rsopt::classes` namespace and implement the trait `MathematicalProgram`.  The trait `MathematicalProgram` dispatches the program to the relevant backend solver _at compile time_.  All solver backends require the `SolverConfig` object for necessary configuration.  Because `SolverConfig` implements trait `Default`, we strongly advise users to use `Default::default()` and adjust only if necessary.

For example, to solve an unconstrained QP, import the following.

```rust
use rsopt::{
    classes::qp::UnconstrainedQuadraticProgram,
    MathematicalProgram,
    SolverConfig,
};
```

You may then construct the program according to its unique constructor.  Various instances of `MathematicalProgram` require different parameters, so we intentionally do not define a unified constructor interface.  For the unconstrained QP example,

```rust
let qp = UnconstrainedQuadraticProgram::new(matrix, vector);
```

Once a `MathematicalProgram` is constructed, you can solve it easily using the `MathematicalProgram::solve()` method.

```rust
let solution = qp.solve(SolverConfig::default());
```

All instances of `MathematicalProgram` return a `Solution` struct, containing program status and all relevant primal or dual variables, when appropriate.  Refer to the project documentation for more details about solver configuration and the `Solution` struct.

## Building RSOPT

To build RSOPT, you will need an up-to-date installation of [Cargo](https://github.com/rust-lang/cargo).

__Minimum Supported Rust Version__ (MSRV): __1.89.0__

Clone this GitHub repository.

```bash
git clone https://github.com/RSOPT/rsopt.git
```

Enter the `rsopt` directory.

```bash
cd rsopt/
```

Build the RSOPT solver.

```bash
cargo build --release
```

## Building the Documentation

The __documentation__ of the RSOPT solver provides information on software interfaces, abstractions, and relevant methods.  The documentation does not detail the mathematical details of RSOPT's implementation; please see the _technical report_ for information on the algorithms used.

Currently, documentation is not available on the open Internet.  They will be published soon.  However, after cloning the repository, you may build the documentation locally using the following command.

```bash
cargo doc --open
```

## Reading the Technical Report

A full __technical report__ on the design of the RSOPT solver, as well as detailed descriptions of all supported problem types, is available at [`report/report.pdf`](https://github.com/RSOPT/rsopt/blob/master/report/report.pdf).
