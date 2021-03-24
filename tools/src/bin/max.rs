fn main() {
    proconio::input! {
        n: usize,
    }
    for i in 0..64i64 {
        println!("1 << {} = {}", i, 1i64 << i as i64);
        if 1 << i > n { break };
    }
    println!("{}", 1isize << 50);
}
