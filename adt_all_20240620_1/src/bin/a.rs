use proconio::input;

fn main() {
    input! {
        h: u64,
    }

    println!("{}", ((h * (12800000 + h)) as f64).sqrt());
}
