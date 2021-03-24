fn main() {
    proconio::input! {
        n: usize,
    }
    let mut cnt = 0;
    for i in 1..=999999 {
        let s = i.to_string();
        let ss = format!("{}{}", s, s);
        let num = ss.parse::<usize>().unwrap();
        if num <= n {
            cnt += 1;
        }
    }
    println!("{}", cnt)
}
