fn main() {
    proconio::input! {
	a: usize,
	b: usize,
	c: usize,
    }
    let a = a % 10;
    let b = b % 4;
    let bc = if b == 2 { if c == 1 {2} else {0}} else { b.pow(c as u32 % 2) };
    let bc = if bc == 0 { 4 } else { bc };
    let abc = a.pow(bc as u32) % 10;
    println!("{}", abc);
}
