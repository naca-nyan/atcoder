
fn main() {
    proconio::input! {
        n: usize,
        abc: [[usize; 3]; n],
    };
    let mut abciter = abc.iter();
    let mut h = abciter.next().unwrap().clone();
    for abc in abciter {
        let mut hs = Vec::new();
        for i in 0..3 {
            hs.push({
                let mut hs = Vec::new();
                for j in 0..3 {
                    if i != j { hs.push(h[j] + abc[i]) }
                }
                *hs.iter().max().unwrap()
            });
        }
        eprintln!("{:?}", hs);
        h = hs;
    }
    println!("{}", *h.iter().max().unwrap());
}
