fn main() {
    proconio::input! {
        n: usize,
        abc: [[usize; 3]; n],
    };
    let mut sum = 0;
    let mut h = Vec::new();
    h.push(abc[0]);
    for i in 0..n {
        sum += abc[i].iter().max().unwrap();
    }
    println!("{}", sum);
}
