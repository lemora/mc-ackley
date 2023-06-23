use crate::{AckleyMcParams, AckleyMcResult};
use float_cmp::ApproxEq;
use plotly::{
    common::{Marker, Mode, Title},
    histogram::Bins,
    layout::{Axis, Layout},
    Histogram, ImageFormat, Plot, Scatter, Scatter3D,
};

// -------------------- plot pure ackley data --------------------

#[allow(dead_code)]
pub fn plot_ackley_2d(
    x1vals: Vec<f32>,
    x2vals: Vec<f32>,
    zvals: Vec<f32>,
    filename: String,
    title: String,
) {
    let trace = Scatter3D::new(x1vals, x2vals, zvals)
        .name("Ackley")
        .mode(Mode::Markers)
        .marker(Marker::new().size(1));

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .title(Title::new(&title))
        .x_axis(Axis::new().title("x ".into()))
        .y_axis(Axis::new().title("y".into()))
        .z_axis(Axis::new().title("z".into()));
    plot.set_layout(layout);

    // Fancy browser-based interactive plot :)
    plot.show();

    plot.write_image(filename.clone(), ImageFormat::PNG, 1000, 1200, 1.0);
    println!("Plot has been saved to {}", &filename);
}

#[allow(dead_code)]
pub fn plot_ackley_1d(xvals: Vec<f32>, yvals: Vec<f32>, filename: String, title: String) {
    let trace = Scatter::new(xvals, yvals)
        .mode(Mode::Markers)
        .name("Ackley");

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .title(Title::new(&title))
        .x_axis(Axis::new().title("x".into()))
        .y_axis(Axis::new().title("y".into()));
    plot.set_layout(layout);

    plot.write_image(filename.clone(), ImageFormat::PNG, 1200, 800, 1.0);
    println!("Plot has been saved to {}", &filename);
}

// -------------------- plot ackley mc data --------------------

pub fn plot_amc_results(params: AckleyMcParams, res: AckleyMcResult) {
    let fname_img: &str = params.foutname.strip_suffix(".csv").unwrap();
    let param_str: String;
    if params.ini_temp.approx_eq(params.final_temp, (0.0, 2)) {
        param_str = format!("(x_delta: {}; T: {})", params.x_delta, params.ini_temp);
    } else {
        param_str = format!(
            "(x_delta: {}; T: {} -> {})",
            params.x_delta, params.ini_temp, params.final_temp
        );
    }

    // Plot energy.
    plot_energies(
        res.e_vals.clone(),
        format!("{}_energies.png", fname_img).as_str(),
        format!("Energy/Cost {}", param_str).as_str(),
    );

    // Plot x-coordinates.
    plot_xcoords(
        res.x_vals.clone(),
        format!("{}_xvals.png", fname_img).as_str(),
        format!(
            "X-Trajectory, {} Dimensions {}",
            params.x_ini.len(),
            param_str
        )
        .as_str(),
    );

    // Create histogram of x-values for one dimension.
    let x_1d: Vec<f32> = res
        .x_vals
        .clone()
        .iter()
        .map(|s| s[0])
        .collect::<Vec<f32>>();

    let bins: usize = 50;
    plot_xval_hist(
        x_1d.clone(),
        bins,
        format!("{}_xhist_{}bins.png", fname_img, bins).as_str(),
        format!(
            "Histogram of x-Values In One Dimension, {bins} Bins {}",
            param_str
        )
        .as_str(),
    );
}

#[allow(dead_code)]
pub fn plot_energies(yvals: Vec<f32>, filename: &str, title: &str) {
    let xvals: Vec<i32> = (0..yvals.len() as i32).collect();

    let trace = Scatter::new(xvals, yvals).mode(Mode::Lines).name("energy");

    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .title(Title::new(&title))
        .x_axis(Axis::new().title("step".into()))
        .y_axis(Axis::new().title("energy (arb. units)".into()));
    plot.set_layout(layout);

    plot.write_image(filename.clone(), ImageFormat::PNG, 1200, 800, 1.0);
    println!("Plot has been saved to {}", &filename);
}

#[allow(dead_code)]
pub fn plot_xcoords(yvals: Vec<Vec<f32>>, filename: &str, title: &str) {
    let xvals: Vec<i32> = (0..yvals.len() as i32).collect();
    let dim: usize = yvals[0].len();

    let mut plot = Plot::new();
    for i in 0..dim {
        let curr_vals: Vec<f32> = yvals.clone().iter().map(|s| s[i]).collect::<Vec<f32>>();
        let trace = Scatter::new(xvals.clone(), curr_vals)
            .mode(Mode::Lines)
            .name(format!("x{i}"));

        plot.add_trace(trace);
    }

    let layout = Layout::new()
        .title(Title::new(&title))
        .x_axis(Axis::new().title("step".into()))
        .y_axis(Axis::new().title("x values".into()));
    plot.set_layout(layout);

    plot.write_image(filename.clone(), ImageFormat::PNG, 1200, 800, 1.0);
    println!("Plot has been saved to {}", &filename);
}

pub fn plot_xval_hist(vals: Vec<f32>, buckets: usize, filename: &str, title: &str) {
    let min_x = vals.clone().iter().copied().reduce(f32::min).unwrap();
    let max_x = vals.clone().iter().copied().reduce(f32::max).unwrap();
    let bucket_size: f32 = (max_x - min_x).abs() / (buckets as f32);

    let t = Histogram::new(vals.clone())
        .auto_bin_x(false)
        .x_bins(Bins::new(min_x as f64, max_x as f64, bucket_size as f64));

    let mut plot = Plot::new();
    plot.add_trace(t);

    let layout = Layout::new()
        .title(Title::new(&title))
        .x_axis(Axis::new().title("x values".into()))
        .y_axis(Axis::new().title("count".into()));
    plot.set_layout(layout);

    plot.write_image(filename.clone(), ImageFormat::PNG, 1200, 800, 1.0);
    println!("Plot has been saved to {}", &filename);
}
