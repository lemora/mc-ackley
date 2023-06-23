use crate::ackley_mc::build_amcparams;
use crate::{AckleyMcParams, AckleyMcResult};
use std::fs;
use std::fs::read_to_string;
use std::io::{BufWriter, Write};

// -------------------- parameter file parser --------------------

pub fn parse_file(filename: String) -> AckleyMcParams {
    let file_contents = read_to_string(filename.clone())
        .expect(format!("Err: Cannot read parameter file '{}'", filename).as_str());
    let lines = file_contents.lines();

    let mut amc_params = build_amcparams();
    for l in lines {
        let parts: Vec<&str> = l.split(' ').collect();
        let key = parts[0];

        if "ini_temp".eq(key) {
            amc_params.ini_temp = parts[1].parse::<f32>().expect("Bad line (ini_temp)");
        } else if "final_temp".eq(key) {
            amc_params.final_temp = parts[1].parse::<f32>().expect("Bad line (final_temp)");
        } else if "n_step".eq(key) {
            amc_params.n_step = parts[1].parse::<u64>().expect("Bad line (n_step)");
        } else if "x_ini".eq(key) {
            let vals: Vec<f32> = parts[1]
                .split(",")
                .filter_map(|s| Option::from(s.parse::<f32>().expect("Bad line (x_ini)")))
                .collect::<Vec<f32>>();
            amc_params.x_ini = vals;
        } else if "x_delta".eq(key) {
            amc_params.x_delta = parts[1].parse::<f32>().expect("Bad line (x_delta)");
        } else if "seed".eq(key) {
            amc_params.seed = parts[1].parse::<u64>().expect("Bad line (seed)");
        } else if "foutname".eq(key) {
            amc_params.foutname = parts[1].parse::<String>().expect("Bad line (foutname)");
        } else {
            panic!("Unknown key '{key}' in file '{filename}'");
        }
    }

    amc_params = validate_amc_params(amc_params);
    return amc_params;
}

fn validate_amc_params(mut params: AckleyMcParams) -> AckleyMcParams {
    assert!(params.n_step > 0);
    assert!(params.ini_temp > 0.0);
    assert!(params.x_delta > 0.0);
    assert!(!params.x_ini.is_empty());

    params.foutname = "out/".to_owned() + &params.foutname;

    // if no/negative final temp was set, assign ini_temp -> no sim. annealing
    if params.final_temp.is_sign_negative() {
        params.final_temp = params.ini_temp;
    }
    return params;
}

// -------------------- write ackley mc results as csv to file --------------------

pub fn create_out_dir() {
    // create 'out' dir if it does not exist
    fs::create_dir_all("out").expect("Cannot create 'out' directory");
}

fn to_csv_line(step: usize, x: Vec<f32>, e: f32) -> String {
    let xstr: String = x.iter().map(|&xi| xi.to_string() + ",").collect();
    return format!("{},{}{}", step, xstr.as_str(), e);
}

#[allow(dead_code)]
pub fn write_res_to_file(res: AckleyMcResult, filename: String) {
    let mut out = BufWriter::new(
        std::fs::File::create(filename.clone())
            .expect(format!("Could not create csv file '{}'", &filename).as_str()),
    );

    let mut step: usize = 0;
    for i in 0..res.accepted_steps.len() {
        step = res.accepted_steps[i] as usize;
        writeln!(
            out,
            "{}",
            to_csv_line(step, res.x_vals[step].clone(), res.e_vals[step])
        )
        .ok();
    }

    // Add line for final step taken if it was not accepted.
    if step < res.n_steps as usize {
        writeln!(
            out,
            "{}",
            to_csv_line(
                res.n_steps as usize,
                res.x_vals[step].clone(),
                res.e_vals[step]
            )
        )
        .ok();
    }
    out.flush()
        .expect(format!("Could not write to csv file '{}'", &filename).as_str());
    println!("CSV data has been saved to {}", &filename);
}
