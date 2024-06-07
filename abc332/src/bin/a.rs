use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        k: usize,
        pq: [(usize, usize); n],
    }
    let mut total = pq.iter().fold(0, |acc, (pi, qi)| acc + pi * qi);
    if total < s {
        total += k;
    }
    println!("{total}");
}
