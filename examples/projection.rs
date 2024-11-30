use std::time::Instant;

use plotters::prelude::*;
use rand::prelude::*;
use zernike;

fn main() {
    let n_xy = 601;
    let n_radial_order = 3;
    let e = [
        0.0,
        0.9199800988318292,
        0.6648755448290419,
        0.8, //// 0.09456405333867661,
        0.9, // 0.5031218038533158,
        0.4, // 0.5929432613175682,
    ];
    let start = Instant::now();
    let zern = zernike::mgs_mode_set(n_radial_order, n_xy);
    println!(" mgs_mode_set in {:?}", start.elapsed());
    //let c: Vec<f64> = (0..n_radial_order*(n_radial_order-1)/2).map(|x| random()).collect();
    let n = n_xy * n_xy;
    let surface: Vec<_> = zern
        .chunks(n)
        .enumerate()
        .fold(vec![0f64; n], |mut a, (i, x)| {
            let c: f64 = e[i]; //random();
            println!("c: {}", c);
            a.iter_mut().zip(x).for_each(|(a, x)| {
                *a += c * x;
            });
            a
        });
    let filename = "examples/surface.png";
    let plot = BitMapBackend::new(&filename, (512, 512)).into_drawing_area();
    plot.fill(&WHITE).unwrap();
    let chart = ChartBuilder::on(&plot)
        .build_cartesian_2d(-1f64..1f64, -1f64..1f64)
        .unwrap();
    let plotting_area = chart.plotting_area();
    let z_max = surface.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let z_min = surface.iter().cloned().fold(f64::INFINITY, f64::min);
    println!("Min-Max: [{:.5};{:.5}]", z_min, z_max);
    let uz: Vec<f64> = surface
        .iter()
        .map(|p| (p - z_min) / (z_max - z_min))
        .collect();
    let d = 2f64 / (n_xy - 1) as f64;
    let h = ((n_xy - 1) / 2) as f64;
    let xy: Vec<_> = (0..n_xy * n_xy)
        .map(|k| {
            let i = (k / n_xy) as f64 - h;
            let j = (k % n_xy) as f64 - h;
            let x = i * d;
            let y = j * d;
            (x, y)
        })
        .collect();
    uz.iter().zip(xy.iter()).for_each(|(z, xy)| {
        plotting_area
            .draw_pixel(*xy, &HSLColor(0.5 * z, 0.5, 0.4))
            .unwrap();
    });
    let start = Instant::now();
    let c_e = zernike::projection(&surface, n_radial_order, n_xy);
    println!("projection in {:?}", start.elapsed());
    println!("<c>: {:?}", c_e);

    // use zernike::filter to remove x tilt
    let d = 2f64 / (n_xy - 1) as f64;
    let h = ((n_xy - 1) / 2) as f64;

    let start = Instant::now();
    let xy: Vec<_> = (0..n_xy * n_xy)
        .map(|k| {
            let i = (k / n_xy) as f64 - h;
            let j = (k % n_xy) as f64 - h;
            let x = i * d;
            let y = j * d;
            (x, y)
        })
        .collect();
    let (xy_tilt_removed, _, _) = zernike::filter_jnm_vec(
        &surface,
        xy.clone().into_iter(),
        vec![
            (1, 0, 0),
            (4, 2, 0),
            (2, 1, 1),
            (3, 1, 1),
            (5, 2, 2),
            (6, 2, 2),
        ],
    );
    println!("filter in {:?}", start.elapsed());

    let filename: &str = "examples/surface_test_remove_xy_tilt.png";
    let plot = BitMapBackend::new(&filename, (512, 512)).into_drawing_area();
    plot.fill(&WHITE).unwrap();
    let chart = ChartBuilder::on(&plot)
        .build_cartesian_2d(-1f64..1f64, -1f64..1f64)
        .unwrap();
    let plotting_area = chart.plotting_area();
    let z_max = xy_tilt_removed
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let z_min = xy_tilt_removed
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    println!("Min-Max: [{:.23};{:.23}]", z_min, z_max);
    //println!("Min-Max: [{:.3};{:.3}]",z_min,z_max);
    let uz: Vec<f64> = xy_tilt_removed
        .iter()
        .map(|p| {
            let diff = z_max - z_min;
            if diff < 1e-10 {
                0.0f64
            } else {
                (p - z_min) / diff
            }
        })
        .collect();
    uz.iter().zip(xy.iter()).for_each(|(z, xy)| {
        plotting_area
            .draw_pixel(*xy, &HSLColor(0.5 * z, 0.5, 0.4))
            .unwrap();
    });
}
