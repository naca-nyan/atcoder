fn ceil(a: usize, b: usize) -> usize {
    (a + (b - 1)) / b
}
#[test]
fn test_ceil() {
    assert_eq!(ceil(7,3), 3);
}
fn main() {
    proconio::input! {
	a: usize,
	b: usize,
	w: usize,
    }
    let w = w * 1000;
    let ns = || (1..=1_000_000)
	.filter(|&i| {
	    let w = w as f64 / i as f64;
	    a as f64 <= w && w <= b as f64
	});
    if let Some(_) = ns().min() {
	println!("{} {}", ns().min().unwrap(), ns().max().unwrap());
    } else {
	println!("UNSATISFIABLE");
    }
}
