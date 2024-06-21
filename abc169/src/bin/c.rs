use proconio::input;

fn main() {
    input! {
        a: u64,
        b: f64,
    }
    let b = (b * 100.0).round() as u64;
    println!("{}", a * b / 100);
}
