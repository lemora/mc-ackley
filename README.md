# Monte Carlo + simulated annealing on Ackley's function

#### Running, the easy way:

- `cargo run -- -h`
- `cargo run <param file name>` // generate csv and plots
- `cargo run <param file name> --justcsv` // only generate csv
- `cargo run <param file name> --noout` // don't generate output

- `cargo run -- --aex` // generate pure ackley examples

An example parameter file can be found in `examples/params`.

#### Testing:

- Run unit tests: `cargo test`
- Run bechmarks: `cargo bench`

#### Building:

Build optimized artifacts (e.g. for external benchmarking): `cargo build -r`

Run the optimized build: `target/release/ackley_mc <param file name> --noout`

### Requirements

- `gnuplot` for the benchmarking library
- `openssl` developer tools
- Helpful: `rustup`
