macro_rules! mylet {
    ($a: ident, $b: expr) => {
	let $a = $b;
    }
}
fn main() {
    let a = 0;
    mylet!(b, 1);
    println!("{} {}", a, b);
}
