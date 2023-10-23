# Metropolis MC + Simulated Annealing on Ackley's Function

A simple program that runs a **Metropolis Monte Carlo** algorithm with or
without **simulated annealing** on **Ackley's function**. It optionally generates an
output csv file and different plots.

### Running, the easy way:

`cargo run <param file name> [options]`

`cargo run -- --aex` // run pure Ackley examples

### Building:

Build and run optimized artifacts (e.g. for external benchmarking):

```
cargo build -r
target/release/ackley_mc <param file name> [options]
```

### Testing/Benchmarking:

Running unit tests: `cargo test`

Running benchmarks: `cargo bench`


### Parameter file format

Each line in the parameter file consists of a name (type string) and
value separated by a whitespace. They are listed in the following:

- **ini_temp**, **final_temp** (float): initial and final temperature. same for no simulated annealing
- **n_step** (integer): number of steps
- **x_ini** (comma-separated list of floats): initial x value for each dimension
- **x_delta** (float): (initial) step size
- **seed** (integer) random number seed for reproducibility
- **foutname** (string): the name of output file(s)

All numeric values need to be positive.
An example parameter file can be found in `examples/params`.

### Requirements

- `gnuplot` for the benchmarking library
- `openssl` developer tools
- Helpful: `rustup`

### Acknowledgements

Based on an assignment by Prof. A. Torda from the Biomolecular Modeling Group at the ZBH, UHH
