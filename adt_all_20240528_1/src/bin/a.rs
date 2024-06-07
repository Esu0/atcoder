use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: marker::Bytes,
    }
    if s[n - 1] == b'o' {
        println!("Yes");
    } else {
        println!("No");
    }
}
