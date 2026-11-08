# RSOPT

<p align="center">
<img src="https://github.com/RSOPT/rsopt/blob/master/rsopt.png" alt="The stylized letters 'RSOPT' on a dark background." width="200" height="200">
</p>

A simple, elegant convex optimization solver.  Powered by the Rust Programming Language.

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

## Technical Report

A full technical report on the design of the RSOPT solver, as well as detailed descriptions of all supported problem types, is available at [`report/report.pdf`](https://github.com/RSOPT/rsopt/blob/master/report/report.pdf).
