use ai::data::get_data;

fn covariance(x: Vec<f64>, y: Vec<f64>) -> f64 {
    let x_mean = x.iter().sum::<f64>() / x.len() as f64;
    let y_mean = y.iter().sum::<f64>() / y.len() as f64;
    let mut sum = 0.0;
    for i in 0..x.len() {
        //println!("x: {}, y: {}", x[i], y[i]);
        //println!("{}", (x[i] - x_mean) * (y[i] - y_mean));
        sum += (x[i] - x_mean) * (y[i] - y_mean);
    }
    println!("x_mean: {}, y_mean: {}", x_mean, y_mean);
    sum / x.len() as f64
}

fn stdev(x: Vec<f64>) -> f64 {
    let mean = x.iter().sum::<f64>() / x.len() as f64;
    let mut sum = 0.0;
    for item in &x {
        sum += (item - mean).powi(2);
    }
    (sum / x.len() as f64).sqrt()
}

fn pearson(x: Vec<f64>, y: Vec<f64>) -> f64 {
    let cov = covariance(x.clone(), y.clone());
    cov / (stdev(x) * stdev(y))
}

fn rand_vector(n: usize, max: f64) -> Vec<f64> {
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(rand::random::<f64>() * max);
    }
    v
}

fn main() {
    let data = get_data();
    let mut all_alpha = Vec::new();
    let mut all_avg = Vec::new();
    let mut all_red = Vec::new();
    let mut all_green = Vec::new();
    let mut all_blue = Vec::new();

    for i in 0..data.0.len() {
        let r = (data.0[i].data[0] + 1.0) * data.2 / 2.0;
        let g = (data.0[i].data[1] + 1.0) * data.2 / 2.0;
        let b = (data.0[i].data[2] + 1.0) * data.2 / 2.0;
        let a = (data.0[i].data[3] + 1.0) * data.2 / 2.0;

        let avg = (r + g + b) / 3.0;
        all_avg.push(avg as f64);
        all_alpha.push(a as f64);

        all_red.push(r as f64);
        all_green.push(g as f64);
        all_blue.push(b as f64);
    }

    println!("Avg: {:?}", all_avg);
    println!("Alpha: {:?}", all_alpha);

    let cov = covariance(all_avg.clone(), all_alpha.clone());
    let pear = pearson(all_avg.clone(), all_alpha.clone());
    println!("corr: {:?}", cov);
    println!("pear: {:?}", pear);

    let rand_pear = pearson(rand_vector(100, 100.0), rand_vector(100, 100.0));
    println!("rand_pear: {:?}", rand_pear);

    let pear_g_r = pearson(all_green.clone(), all_red.clone());
    println!("pear_g_r: {:?}", pear_g_r);

    let pear_g_g = pearson(all_green.clone(), all_green.clone());
    println!("pear_g_g: {:?}", pear_g_g);

    println!("{}", pear * pear);
}
