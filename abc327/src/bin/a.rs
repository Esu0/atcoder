use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    }
    if s.contains("ab") || s.contains("ba") {
        println!("Yes");
    } else {
        println!("No");
    }
}
