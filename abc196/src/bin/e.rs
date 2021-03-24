fn main() {
    proconio::input! {
        n: usize,
        at: [(isize, usize); n],
        q: usize,
        x: [isize; q],
    }
    let inf = 1 << 62 as isize;
    let mut c1 = 0;
    let mut c2 = -inf;
    let mut c3 = inf;
    for (a, t) in at.into_iter() {
        match t {
            1 => {
                c1 += a;
                c2 += a;
                c3 += a;
            },
            2 => c2 = c2.max(a),
            3 => c3 = c3.min(a),
            _ => panic!(""),
        }
    }
    for x in x {
        let res = (x + c1).max(c2).min(c3);
        println!("{}", res);
    }
}
