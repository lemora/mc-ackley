use ackley_mc::ackley_mc::build_amcparams;
use ackley_mc::ackley_mc::{ackley, run_ackley_mc};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

// -------------------- pure ackley benchmarks --------------------

fn cbench_ackley_100k_dim(c: &mut Criterion) {
    let xarr: Vec<f32> = vec![3.1415; 100000];
    c.bench_function("ackley 100k dim", |b| {
        b.iter(|| black_box(ackley(black_box(xarr.clone()))))
    });
}

// -------------------- ackley mc benchmarks --------------------

#[allow(dead_code)]
fn cbench_amc_3d_100k_steps(c: &mut Criterion) {
    // parameters from given example file
    let mut params = build_amcparams();
    params.n_step = 100000;
    params.ini_temp = 0.1;
    params.final_temp = 0.1;
    params.x_ini = vec![-1.5, 0.5, 1.0];
    params.x_delta = 0.5;
    params.seed = 1699;

    c.bench_function("Ackley MC, 3D, 100k steps", |b| {
        b.iter(|| black_box(run_ackley_mc(black_box(params.clone()))))
    });
}

fn cbench_amc_3d_500k_steps(c: &mut Criterion) {
    let mut group = c.benchmark_group("amc 500k");
    group.measurement_time(Duration::from_secs(10));

    // parameters from given example file
    let mut params = build_amcparams();
    params.n_step = 500000;
    params.ini_temp = 0.1;
    params.final_temp = 0.1;
    params.x_ini = vec![-1.5, 0.5, 1.0];
    params.x_delta = 0.5;
    params.seed = 1699;

    group.bench_function("Ackley MC, 3D, 500k steps", |b| {
        b.iter(|| black_box(run_ackley_mc(black_box(params.clone()))))
    });
    group.finish();
}

fn cbench_amc_3d_500k_steps_simanneal(c: &mut Criterion) {
    let mut group = c.benchmark_group("amc 500k");
    group.measurement_time(Duration::from_secs(10));

    // parameters from given example file
    let mut params = build_amcparams();
    params.n_step = 500000;
    params.ini_temp = 0.1;
    params.final_temp = 0.01;
    params.x_ini = vec![-1.5, 0.5, 1.0];
    params.x_delta = 0.5;
    params.seed = 1699;

    group.bench_function("Ackley MC, 3D, 500k steps, sim. annealing", |b| {
        b.iter(|| black_box(run_ackley_mc(black_box(params.clone()))))
    });
    group.finish();
}

// -------------------- running benchmarks --------------------

criterion_group!(
    benches,
    cbench_ackley_100k_dim,
    //cbench_amc_3d_100k_steps,
    cbench_amc_3d_500k_steps,
    cbench_amc_3d_500k_steps_simanneal,
);
criterion_main!(benches);
