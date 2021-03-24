use std::collections::HashSet;
fn main() {
    proconio::input! {
	n: u64,
    }
    let mut expressed = HashSet::new();
    for a in 2..n {
	let mut ab = a * a;
	if ab > n { break };
	while ab <= n {
	    expressed.insert(ab);
	    ab *= a;
	}
    }
    println!("{}", n - expressed.len() as u64);
}
