use proconio::input;

fn main() {
    input! {
        n: usize,
        mut sc: [(String, usize); n],
    }
    let total_rate = sc.iter().map(|(_, c)| c).sum::<usize>();
    sc.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    println!("{}", sc[total_rate % n].0);
}
