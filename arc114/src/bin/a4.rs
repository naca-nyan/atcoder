fn primes(n: usize) -> Vec<usize> {
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    primes.into_iter().filter(|p| n % p == 0).collect()
}
fn main() {
    proconio::input! {
	n: usize,
	mut x: [usize; n]
    }
    let mut ys = vec![1];
    x.sort();
    for x in x.into_iter() {
	let xprimes = primes(x);
	print!("{}: ", x);
	for p in xprimes.iter() {
	    print!("{} ", *p);
	}
	println!("");
	let mut newys = Vec::new();
	for p in xprimes.iter() {
	    for y in ys.iter() {
		if y % p != 0 {
		    newys.push(y * p);
		} else {
		    newys.push(*y);
		}
	    }
	}
	ys = newys;
	println!{"{}: {:?}", x, ys};
    }
    let y = ys.iter().min().unwrap();
    println!("{}", y);
}
