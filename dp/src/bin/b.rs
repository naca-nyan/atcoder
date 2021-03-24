fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        h: [usize; n],
    }
    let mut dp = Vec::new();
    let cost = |i,j| (h[i] as isize - h[j] as isize).abs() as usize;
    dp.push(0);
    eprintln!("{:?}", h);
    for i in 1..n {
        let l = if i < k {0} else {i - k};
        let cs = (l..i).map(|j| dp[j] + cost(j, i));
        dp.push(cs.min().unwrap());
    }
    let res = dp[n - 1];
    println!("{}", res);
}
