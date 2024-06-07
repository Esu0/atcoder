use proconio::input;

fn main() {
    input! {
        n: u32,
        s: u32,
    }
    let ans = (1..=n).flat_map(|i| (1..=n).map(move |j| (i, j))).filter(|&(i, j)| i + j <= s).count();
    println!("{}", ans);
}
