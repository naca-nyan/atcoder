fn main() {
    proconio::input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let v = 100 * 1000 + 1;
    let mut dp = vec![std::usize::MAX >> 2; v+1];
    dp[0] = 0;
    for wv in wv.iter() {
        let mut d = vec![0; v+1];
        for v in 0..=v {
            if v < wv.1 {
                d[v] = dp[v];
            } else {
                let w1 = dp[v - wv.1] + wv.0;
                let w2 = dp[v];
                d[v] = w1.min(w2);
            }
        }
        dp = d;
    }
    let mut res = v;
    while dp[res] > w { res -= 1 }
    println!("{:?}", res);
}
