use proconio::input;
fn main() {
    input!{
	n: u32,
	mut p: [(i32, i32 ,i32); n]
    }
    p.insert(0, (0, 0, 0));
    let mut res = true;
    for i in 0..n as usize {
	let (t0, x0, y0) = p[i];
	let (t1, x1, y1) = p[i + 1];
	let dt = t1 - t0;
	let d = (x1 - x0).abs() + (y0 - y1).abs();
	if !(d <= dt && dt % 2 == d % 2) {
	    res = false;
	    break;
	}
    }
    println!("{}", if res {"Yes"} else {"No"})
}
