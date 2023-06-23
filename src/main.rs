use clap::Parser;
use std::error::Error;

use crate::ackley_mc::run_ackley_mc;
use crate::ackley_mc::AckleyMcParams;
use crate::ackley_mc::AckleyMcResult;

mod ackley_examples;
mod ackley_mc;
mod fileio;
mod plot;

// -------------------- main --------------------

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    if args.filename.is_some() {
        let param_filename = args.filename.unwrap();
        let amc_params: AckleyMcParams = fileio::parse_file(param_filename);

        let res: AckleyMcResult = run_ackley_mc(amc_params.clone());

        println!(
            "--> Accepted {} of {} steps.",
            res.accepted_steps.len() - 1, // correct for accepting initial value
            res.n_steps
        );
        println!("--> Best energy: {}  at {:?}", res.e_best, res.x_best);

        if args.noout {
            return Ok(());
        }

        fileio::create_out_dir();
        // Create csv file.
        fileio::write_res_to_file(res.clone(), amc_params.foutname.clone());

        if args.justcsv {
            return Ok(());
        }

        plot::plot_amc_results(amc_params.clone(), res.clone());
        return Ok(());
    }
    if args.aex {
        println!("\nGenerating pure Ackley function examples...");
        ackley_examples::generate();
        return Ok(());
    }
    return Err("No parameter file name or 'aex' argument provided.".into());
}

// -------------------- command line parser --------------------

/// Simple program doing Ackley + Monte Carlo things.
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
#[command(next_line_help = true)]
#[command(arg_required_else_help = true)]
struct Args {
    /// Name of the parameter file. Will normally generate csv and plots.
    filename: Option<String>,

    /// Just run Ackley Monte Carlo, don't generate output.
    #[arg(long, default_value_t = false)]
    noout: bool,

    /// Run Ackley Monte Carlo and generate a csv file.
    #[arg(long, default_value_t = false)]
    justcsv: bool,

    /// Generate Ackley example plots + csv (only if no file name provided).
    #[arg(long, default_value_t = false)]
    aex: bool,
}
