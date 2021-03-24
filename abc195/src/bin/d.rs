fn main() {
    proconio::input! {
	n: usize,
	m: usize,
	q: usize,
	wv: [(usize,usize); n],
	x: [usize; m],
	lr: [(usize, usize); q],
    }
    for (l, r) in lr.iter() {
	x.remove(l)
    }
}
