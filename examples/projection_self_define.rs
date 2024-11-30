use plotters::prelude::*;
use rand::prelude::*;
use zernike;

fn main() {
    let n_xy = 683;
    let n_radial_order = 8;
    let c_e = [
        -174.3369459869151,
        -2539.0933690188835,
        1141.4269904696291,
        964.511295334499,
        2.733751192809555,
        2.008347168997781,
        0.04793141820689858,
        -1.8934813210931922,
        -1.3354654270086483,
        0.04387892954131445,
        0.4166436780238098,
        -0.7085029356027714,
        -0.4936506782324884,
        -0.8152366771746395,
        -1.8764553208824288,
        -0.7050409536457476,
        -0.4063017397496247,
        -0.4578997288301484,
        0.8237691426114832,
        1.1521131284483952,
        1.0715621968724955,
        0.11102980714178678,
        0.16306140461187837,
        0.6459308730598172,
        0.15418261674060707,
        0.371090533509542,
        -0.14222918770095558,
        -0.2543749759883439,
        -0.19303429179850984,
        0.311414898335151,
        -0.14424084838877776,
        0.076744718552128,
        0.09911286358297346,
        -0.07175053913196831,
        0.010297188652767264,
        -0.286770985206802,
    ];
    let zern = zernike::mgs_mode_set(n_radial_order, n_xy);
    //let c: Vec<f64> = (0..n_radial_order*(n_radial_order-1)/2).map(|x| random()).collect();
    let n = n_xy * n_xy;
    let surface: Vec<_> = zern
        .chunks(n)
        .enumerate()
        .fold(vec![0f64; n], |mut a, (i, x)| {
            let c: f64 = c_e[i];
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
    println!("Min-Max: [{:.3};{:.3}]", z_min, z_max);
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

    let c_e = zernike::projection(&surface, n_radial_order, n_xy);
    println!("<c>: {:?}", c_e)
}
