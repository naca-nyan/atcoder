fn primes(n: usize) -> Vec<usize> {
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    primes.into_iter().filter(|p| n % p == 0).collect()
}
fn main() {
    proconio::input! {
	n: usize,
	mut x: [usize; n]
    }
    let mut y = 1;
    x.sort();
    for x in x.into_iter() {
	let xprimes = primes(x);
	print!("{}: ", x);
	for p in xprimes.iter() {
	    print!("{} ", *p);
	}
	println!("");
	if !xprimes.iter().any(|&p| y % p == 0) {
	    y *= xprimes[0];
	}
    }
    println!("{}", y);
}
