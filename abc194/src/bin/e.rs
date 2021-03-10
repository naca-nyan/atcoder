fn mex(n: usize, x: &[usize]) -> usize {
    (0..n).filter(|i|!x.contains(i)).min().unwrap()
}
#[test]
fn test_mex() {
    let x = vec![1, 2, 3];
    let n = 3;
    assert_eq!(mex(n, &x), 0);
}
fn main() {
    proconio::input! {
	n: usize,
	m: usize,
	a: [usize; n],
    }
    let res = a.windows(m).map(|w|mex(n, w)).min().unwrap();
    println!{"{}", res};
}
