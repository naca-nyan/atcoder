fn point(cards: &[u8]) -> usize {
    let c = |i| cards.iter().filter(|&c| *c == i).count() as u32;
    (1..=9).map(|i| i as usize * 10usize.pow(c(i))).sum()
}
fn main() {
    proconio::input! {
        k: usize,
        s: proconio::marker::Bytes,
        t: proconio::marker::Bytes,
    }
    let c2n = |&c| { if c == b'#' { 0 } else { c - b'0' }};
    let mut s: Vec<u8> = s.iter().map(c2n).collect();
    let mut t: Vec<u8> = t.iter().map(c2n).collect();
    let mut c = vec![0; 10];
    for i in 0..4 {
        c[s[i] as usize] += 1;
        c[t[i] as usize] += 1;
    }

    let mut nom = 0;
    for i in 1..=9 {
        for j in 1..=9 {
            if c[i] == k || c[j] == k { continue };
            s[4] = i as u8;
            t[4] = j as u8;
            if point(&s) > point(&t) {
                let inom = k - c[i];
                let jnom = if i == j { k - c[j] - 1 } else { k - c[j] };
                nom += inom * jnom;
            }
        }
    }
    let denom = (9 * k - 8) * (9 * k - 9);
    println!("{}", nom as f64 / denom as f64);
}
