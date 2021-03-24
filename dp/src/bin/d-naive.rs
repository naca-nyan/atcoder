fn solve(n: usize, w: usize, wv: &[(usize, usize)]) -> usize {
    if n == 0 { return 0 }
    let i = n - 1;
    let rec = |n, w| solve(n, w, wv);
    let wv = wv[i];
    if w < wv.0 {
        return rec(i, w);
    }
    let v1 = rec(i, w - wv.0) + wv.1;
    let v2 = rec(i, w);
    v1.max(v2)
}
fn main() {
    proconio::input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let res = solve(n, w, &wv);
    println!("{}", res);
}
