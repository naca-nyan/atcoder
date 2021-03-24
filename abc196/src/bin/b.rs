fn main() {
    proconio::input! {
        x: proconio::marker::Bytes,
    }
    let s: Vec<u8> = x.into_iter().take_while(|&c| c != b'.').collect();
    println!("{}", std::str::from_utf8(&s).unwrap());
}
