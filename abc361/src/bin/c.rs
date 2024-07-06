use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u64; n],
    }

    let l = n - k;
    a.sort_unstable();
    let ans = (l - 1..n).map(|i| a[i] - a[i - (l - 1)]).min().unwrap();
    println!("{}", ans);
}
