use rand::{thread_rng, Rng};

fn monte_carlo(f: fn(&[f64]) -> f64, a: &[f64], b: &[f64], n: u32) -> f64 {
    (0..n)
        .map(|_| {
            a.iter()
                .zip(b.iter())
                .map(|(a_i, b_i)| {
                    thread_rng().gen_range(if a_i <= b_i { *a_i..=*b_i } else { *b_i..=*a_i })
                })
                .collect::<Vec<_>>()
        })
        .map(|x| f(&x))
        .sum::<f64>()
        * a.iter()
            .zip(b.iter())
            .map(|(a_i, b_i)| b_i - a_i)
            .product::<f64>()
        / n as f64
}

fn f(x: &[f64]) -> f64 {
    x[0] * x[1]
}

fn main() {
    let r = monte_carlo(f, &[0.0, 0.0], &[1.0, 2.0], 100000);
    println!("Result: {}", r);
}
