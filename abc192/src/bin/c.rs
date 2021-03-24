fn f(n: usize) -> usize {
    let mut chars = n.to_string().chars().collect::<Vec<char>>();
    chars.sort();
    let g1: usize = chars.iter().rev().collect::<String>().parse().unwrap();
    let g2: usize = chars.iter()      .collect::<String>().parse().unwrap();
    g1 - g2
}
fn main() {
    proconio::input! {
        n: usize,
        k: usize,
    }
    let res = (0..k).fold(n, |x, _| f(x));
    println!("{}", res);
}
