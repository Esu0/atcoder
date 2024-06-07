use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut indexed = a
        .into_iter()
        .enumerate()
        .map(|(i, ai)| (i, ai, 0i64))
        .collect::<Vec<_>>();
    indexed.sort_unstable_by_key(|&(_, ai, _)| ai);
    let mut s0 = 0;
    let mut s = 0;
    let mut prev = i64::MAX;
    for (_, ai, sum) in indexed.iter_mut().rev() {
        let ai = *ai;
        if ai < prev {
            *sum = s;
            s0 = s;
            prev = ai;
        } else {
            *sum = s0;
        }
        s += ai;
    }
    indexed.sort_unstable_by_key(|&(i, _, _)| i);
    let mut iter = indexed.iter().map(|&(_, _, sum)| sum);
    if let Some(b1) = iter.next() {
        print!("{b1}");
    }
    for bi in iter {
        print!(" {bi}");
    }
    println!();
}
