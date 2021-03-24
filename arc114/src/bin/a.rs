use std::collections::HashSet;
fn primes(n: usize) -> Vec<usize> {
    let prime = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    prime.into_iter().filter(|p| n % p == 0).collect()
}
fn main() {
    proconio::input! {
	n: usize,
	mut x: [usize; n]
    }
    let mut ys = HashSet::new();
    ys.insert(1);
    x.sort();
    for x in x.into_iter() {
	let xprimes = primes(x);
	let mut newys = HashSet::new();
	for p in xprimes.iter() {
	    for y in ys.iter() {
		if y % p != 0 {
		    newys.insert(y * p);
		} else {
		    newys.insert(*y);
		}
	    }
	}
	ys = newys;
    }
    let y = ys.iter().min().unwrap();
    println!("{}", y);
}
