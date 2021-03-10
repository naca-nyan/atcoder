fn harmonic(n: u32) -> f64 {
    (1..n).map(|i| 1.0/i as f64).sum()
}

fn main() {
    proconio::input! { n: u32 };
    let res = n as f64 * harmonic(n);
    println!("{}", res);
}
