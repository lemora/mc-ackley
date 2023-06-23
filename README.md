# Monte Carlo + simulated annealing on Ackley's function

A simple program that runs a **Metropolis Monte Carlo** simulation with or
without **simulated annealing** on **Ackley's function**. It optionally generates an
output csv file and different plots.

#### Running, the easy way:

- `cargo run -- -h`
- `cargo run <param file name>` // generate csv and plots
- `cargo run <param file name> --justcsv` // only generate csv
- `cargo run <param file name> --noout` // don't generate output

- `cargo run -- --aex` // generate pure Ackley example plots

An example parameter file can be found in `examples/params`.

#### Testing:

- Run unit tests: `cargo test`
- Run benchmarks: `cargo bench`

#### Building:

Build optimized artifacts (e.g. for external benchmarking): `cargo build -r`

Run the optimized build: `target/release/ackley_mc <param file name> --noout`


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

### Requirements

- `gnuplot` for the benchmarking library
- `openssl` developer tools
- Helpful: `rustup`
