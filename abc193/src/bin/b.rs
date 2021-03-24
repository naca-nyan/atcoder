fn main() {
    proconio::input! {
	n: usize,
	d: [(usize, usize, usize); n]
    }
    if let Some(t) = d.iter().filter(|t| t.0 < t.2).map(|t| t.1).min() {
        println!("{}", t)
    } else {
        println!("-1")
    }
}
