use float_cmp::ApproxEq;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::f32::consts::E;
use std::f32::consts::PI;

// -------------------- ackley --------------------

static ACK_A: f32 = 20.0;
static ACK_B: f32 = 0.2;
static ACK_C: f32 = 2.0 * PI;

#[inline]
pub fn ackley(x: Vec<f32>) -> f32 {
    let n = x.len();

    let mut square_sum: f32 = 0.0;
    let mut cosine_sum: f32 = 0.0;
    for i in 0..n {
        square_sum += x[i] * x[i];
        cosine_sum += (ACK_C * x[i]).cos();
    }

    let mut y: f32 = 0.0;
    y += -ACK_A * (-ACK_B * (1.0 / (n as f32) * square_sum).sqrt()).exp();
    y -= (1.0 / (n as f32) * cosine_sum).exp();
    y += ACK_A + E;

    return y;
}

// -------------------- monte carlo --------------------

#[inline]
pub fn run_ackley_mc(params: AckleyMcParams) -> AckleyMcResult {
    let mut rng = ChaCha8Rng::seed_from_u64(params.seed);

    // temperature things for simulated annealing
    let mut temperature: f32 = params.ini_temp;
    let anneal: bool = !params.ini_temp.approx_eq(params.final_temp, (0.0, 2));
    let c_mult: f32 = (1.0 / (params.n_step as f32) * (params.final_temp / temperature).ln()).exp();

    let mut xvec: Vec<f32> = params.x_ini.clone();
    let mut energy: f32 = ackley(xvec.clone());

    let mut e_best: f32 = energy;
    let mut x_best: Vec<f32> = xvec.clone();

    // Initialise result struct.
    let mut amc_res: AckleyMcResult = build_amcresult();
    amc_res.n_steps = params.n_step;
    amc_res.x_vals.push(xvec.clone());
    amc_res.e_vals.push(energy.clone());

    for step in 0..params.n_step {
        let x_trial: Vec<f32> = get_trial_x(xvec.clone(), params.x_delta, &mut rng);
        let e_trial: f32 = ackley(x_trial.clone());

        if e_trial < e_best {
            e_best = e_trial;
            x_best = x_trial.clone();
        }

        // evaluate whether to accept the current step
        let accept: bool;
        if e_trial <= energy {
            accept = true;
        } else {
            let delta_e: f32 = energy - e_trial;
            accept = rng.gen_range(0.0..1.0) < (delta_e / temperature).exp();
        }
        if accept {
            xvec = x_trial.clone();
            energy = e_trial;

            amc_res.accepted_steps.push(step);
        }

        amc_res.x_vals.push(xvec.clone());
        amc_res.e_vals.push(energy);

        if anneal {
            temperature *= c_mult;
        }
    }

    amc_res.e_best = e_best;
    amc_res.x_best = x_best;
    return amc_res;
}

#[inline]
fn get_trial_x(x: Vec<f32>, x_delta: f32, rng: &mut ChaCha8Rng) -> Vec<f32> {
    let dim: u32 = x.len() as u32;
    let idim = rng.gen_range(0..dim) as usize;
    let step: f32 = x_delta * (2.0 * rng.gen_range(0.0..1.0) - 1.0);

    let mut x_trial: Vec<f32> = x.clone();
    x_trial[idim] += step;

    return x_trial;
}

// -------------------- input and output ackley mc structs --------------------

#[derive(Clone, Debug)]
pub struct AckleyMcParams {
    pub ini_temp: f32,
    pub final_temp: f32,
    pub n_step: u64,
    pub x_ini: Vec<f32>,
    pub x_delta: f32,
    pub seed: u64,
    pub foutname: String,
}

pub fn build_amcparams() -> AckleyMcParams {
    let amc_params = AckleyMcParams {
        ini_temp: 0.1,
        final_temp: -1.0,
        n_step: 100,
        x_ini: vec![0.0],
        x_delta: 0.0,
        seed: 3141,
        foutname: "ackley_mc_out.csv".to_string(),
    };
    return amc_params;
}

#[derive(Clone, Debug)]
pub struct AckleyMcResult {
    pub n_steps: u64,
    pub accepted_steps: Vec<u64>,
    pub x_vals: Vec<Vec<f32>>,
    pub e_vals: Vec<f32>,
    pub x_best: Vec<f32>,
    pub e_best: f32,
}

pub fn build_amcresult() -> AckleyMcResult {
    let amc_res = AckleyMcResult {
        n_steps: 0,
        accepted_steps: vec![],
        x_vals: vec![],
        e_vals: vec![],
        x_best: vec![],
        e_best: 500_000.0,
    };
    return amc_res;
}

// -------------------- unit tests --------------------

#[test]
fn test_ackley_atzero_3d() {
    let res = ackley(vec![0.0, 0.0, 0.0]);
    assert!((res - 0.0).abs() <= 0.0001); // epsilon-like
}
