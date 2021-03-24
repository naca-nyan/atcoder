fn main() {
    proconio::input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let mut dp = vec![0; w + 1];
    for wv in wv.iter() {
        let mut d = Vec::new();
        for w in 0..=w {
            if w < wv.0 {
                d.push(dp[w]);
            } else {
                let v1 = dp[w - wv.0] + wv.1;
                let v2 = dp[w];
                d.push(v1.max(v2));
            }
        }
        dp = d;
    }
    eprintln!("{:?}", dp);
    println!("{}", dp[w]);
}
