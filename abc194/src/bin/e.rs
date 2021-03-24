fn main() {
    proconio::input! {
    n: usize,
    m: usize,
    a: [usize; n],
    }
    let mut pos = vec![vec![0]; n + 1];
    for (i, &a) in a.iter().enumerate() {
        pos[a].push(i + 1);
    }
    for p in pos.iter_mut() {
        p.push(n + 1);
    }
    let res = pos
        .iter()
        .enumerate()
        .filter(|(_, p)| p.windows(2).any(|w| w[0] + m < w[1]))
        .map(|t| t.0)
        .min()
        .unwrap();
    println!("{}", res);
}
