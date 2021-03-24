fn main() {
    proconio::input! {
        s: String,
    }
    let res = s.chars().enumerate().all(|(i, c)| {
        match i % 2 {
            0 => c.is_lowercase(),
            1 => c.is_uppercase(),
            _ => panic!(),
        }
    });
    println!("{}", if res { "Yes" } else { "No" });
}
