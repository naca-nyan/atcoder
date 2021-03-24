fn expressed(a: u64, b: u64) -> bool {
    let logb_a = b as u32;
    (2..logb_a).any(|i| b.pow(i) == a)
}
fn main() {
    proconio::input! {
	n: u64,
    }
    let mut cnt = 0;
    let mut base = Vec::new();
    for a in 2..n {
	if a * a > n { break };
	if base.iter().any(|&b| expressed(a, b)) { break };
	let loga_n = (n as f64).log(a as f64);
	cnt = loga_n as u64 - 1;
	base.push(a);
    }
    println!("{}", n - cnt);
}
