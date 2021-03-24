fn main() {
    proconio::input! {
	n: usize,
	x: [usize; n]
    }
    let mut y = 1;
    let coprime = |x, y| {
	num::integer::gcd(x, y) == 1
    };
    loop {
	if x.iter().all(|&x|!coprime(x, y)) {
	    break;
	}
	y += 1;
    }
    println!("{}", y);
}
