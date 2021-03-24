fn commas(n: u64) -> u64 {
    if n == 0 { return 0; }
    let mut n = n;
    let mut c = 0;
    while n != 0 {
	n /= 10;
	c += 1;
    }
    (c - 1) / 3
}
fn main() {
    proconio::input!{
	n: u64,
    }
    let mut cnt = 0;
    let mut bias = 0;
    for i in 0..commas(n) {
	bias += 999 * 10u64.pow(3*i as u32);
	if i > 0 {
	    cnt += i * (999 * 10u64.pow(3*i as u32));
	}
    }
    let ans = cnt + commas(n) * (n - bias);
    println!("{}", ans);
}
#[test]
fn commas_test() {
    for i in 0..1_000 {
	assert_eq!(commas(i), 0);
    };
    assert_eq!(commas(  999), 0);
    assert_eq!(commas(1_000), 1);
    assert_eq!(commas(  999_999), 1);
    assert_eq!(commas(1_000_000), 2);
    assert_eq!(commas(  999_999_999), 2);
    assert_eq!(commas(1_000_000_000), 3);
}
