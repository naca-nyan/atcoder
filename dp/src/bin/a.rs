fn main() {
    proconio::input! {
	n: usize,
	h: [usize; n],
    }
    let mut dp = Vec::new();
    let cost = |i,j| (h[i] as isize - h[j] as isize).abs() as usize;
    dp.push(0);
    dp.push(cost(0, 1));
    for i in 2..n {
	let c1 = dp[i - 1] + cost(i - 1, i);
	let c2 = dp[i - 2] + cost(i - 2, i);
	dp.push(c1.min(c2));
    }
    let res = dp[n - 1];
    println!("{}", res);
}

