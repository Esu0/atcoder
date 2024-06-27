use proconio::{input, marker};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            s: marker::Bytes,
            x: [u32; n - 1],
        }
        
    }
}
