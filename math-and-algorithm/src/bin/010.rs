use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut ans = 1;
    for i in 1..=n {
        ans *= i;
    }
    println!("{}", ans);
}