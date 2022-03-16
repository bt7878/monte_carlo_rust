use rand::{thread_rng, Rng};

fn monte_carlo(f: fn(&Vec<f64>) -> f64, a: &[f64], b: &[f64], n: i32) -> Option<f64> {
    if a.len() != b.len() {
        return None;
    }

    let mut w = 1.0;

    for i in 0..a.len() {
        w *= b[i] - a[i];
    }

    let mut x_d: Vec<f64> = vec![0.0; a.len()];
    let mut sum = 0.0;

    for _ in 0..n {
        for i in 0..x_d.len() {
            let range = if b[i] >= a[i] {
                a[i]..=b[i]
            } else {
                b[i]..=a[i]
            };
            x_d[i] = thread_rng().gen_range(range);
        }
        sum += f(&x_d);
    }

    Some(sum * w / n as f64)
}

fn f(x: &Vec<f64>) -> f64 {
    x[0] * x[1]
}

fn main() {
    let r = monte_carlo(f, &[0.0, 0.0], &[1.0, 2.0], 1000);

    println!("Result: {}", r.unwrap());
}
