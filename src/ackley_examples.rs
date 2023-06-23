use crate::plot::plot_ackley_1d;
use crate::plot::plot_ackley_2d;
use ackley_mc::ackley_mc::ackley;
use std::fs;

use itertools_num::linspace;
use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
pub fn generate() {
    fs::create_dir_all("out").expect("Cannot create 'out' directory");

    let steps: i32 = 150;
    let range: f32 = 4.0;

    let res1 = ackley_loop_1d(steps, range, true);
    let filename = format!("out/ackley_1d_{steps}steps.png");
    let caption = format!("Ackley in 1D, {steps} Steps");
    plot_ackley_1d(res1.0, res1.1, filename, caption);

    let res2 = ackley_loop_2d(steps, range, true);
    let filename = format!("out/ackley_3d_{steps}steps.png");
    let caption = format!("Ackley in 3D, {steps} Steps (per dimension)");
    plot_ackley_2d(res2.0, res2.1, res2.2, filename, caption);

    let res3 = ackley_loop_5d_vary1(steps, range, true);
    let filename = format!("out/ackley_5d_vary1_{steps}steps.png");
    let caption = format!("Ackley in 5D (all but one set to 1.0), {steps} Steps");
    plot_ackley_1d(res3.0, res3.1, filename, caption);
}

// -------------------- examples --------------------

#[allow(dead_code)]
fn ackley_loop_1d(steps: i32, abs_bound: f32, csv: bool) -> (Vec<f32>, Vec<f32>) {
    let mut x_vec: Vec<f32> = Vec::new();
    let mut y_vec: Vec<f32> = Vec::new();

    let path = format!("out/ackley_1d_{steps}steps.csv");
    let mut buffer = File::create(path).unwrap();

    let xrange = linspace::<f32>(-abs_bound as f32, abs_bound as f32, steps as usize);
    for i in xrange.into_iter() {
        let x: f32 = i as f32;
        let y: f32 = ackley(vec![x]);

        x_vec.push(x);
        y_vec.push(y);

        if csv {
            write!(buffer, "{},{}\n", x, y).ok();
        }
    }

    return (x_vec, y_vec);
}

#[allow(dead_code)]
fn ackley_loop_5d_vary1(steps: i32, abs_bound: f32, csv: bool) -> (Vec<f32>, Vec<f32>) {
    let mut x1_vec: Vec<f32> = Vec::new();
    let mut y_vec: Vec<f32> = Vec::new();

    let const_val: f32 = 1.0;

    let path = format!("out/ackley_5d_vary1_{steps}steps.csv");
    let mut buffer = File::create(path).unwrap();

    let xrange = linspace::<f32>(-abs_bound as f32, abs_bound as f32, steps as usize);
    for i in xrange.clone().into_iter() {
        let y: f32 = ackley(vec![i as f32, const_val, const_val, const_val, const_val]);
        x1_vec.push(i as f32);
        y_vec.push(y);

        if csv {
            write!(
                buffer,
                "{},{},{},{},{},{}\n",
                i, const_val, const_val, const_val, const_val, y
            )
            .ok();
        }
    }

    return (x1_vec, y_vec);
}

#[allow(dead_code)]
fn ackley_loop_2d(steps: i32, abs_bound: f32, csv: bool) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut x1_vec: Vec<f32> = Vec::new();
    let mut x2_vec: Vec<f32> = Vec::new();
    let mut y_vec: Vec<f32> = Vec::new();

    let path = format!("out/ackley_2d_{steps}steps.csv");
    let mut buffer = File::create(path).unwrap();

    let xrange = linspace::<f32>(-abs_bound as f32, abs_bound as f32, steps as usize);
    for i in xrange.clone().into_iter() {
        for j in xrange.clone().into_iter() {
            let y: f32 = ackley(vec![i as f32, j as f32]);
            x1_vec.push(i as f32);
            x2_vec.push(j as f32);
            y_vec.push(y);

            if csv {
                write!(buffer, "{},{},{}\n", i, j, y).ok();
            }
        }
    }

    return (x1_vec, x2_vec, y_vec);
}
