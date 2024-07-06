use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    match &s[..] {
        b"RMS" | b"RSM" | b"SRM" => println!("Yes"),
        _ => println!("No"),
    }
}
