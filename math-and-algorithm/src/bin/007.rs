use proconio::input;

fn main() {
    input! {
        n: u32,
        x: u32,
        y: u32,
    }

    let ans = (1..=n).filter(|&i| i % x == 0 || i % y == 0).count();
    println!("{}", ans);
}
