fn main() {
    proconio::input! {
        s: proconio::marker::Bytes,
        t: proconio::marker::Bytes,
    }
    let slen = s.len();
    let tlen = t.len();
    let mut dp = vec![vec![0; slen + 1]; tlen + 1];
    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    for d in dp.iter() {
        eprintln!("{:?}", d);
    }
    let mut len = dp[s.len()][t.len()];
    let mut i = s.len();
    let mut j = t.len();
    let mut lcs = vec![0; len + 1];
    while len > 0 {
        if s[i - 1] == t[i - 1] {
            lcs[len] = s[i - 1];
            len -= 1;
            i -= 1;
            j -= 1;
        } else if dp[i][j] == dp[i - 1][j] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    println!("{}", std::str::from_utf8(&lcs).unwrap());
}
