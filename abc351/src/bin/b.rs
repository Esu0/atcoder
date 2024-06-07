use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        a: [Bytes; n],
        b: [Bytes; n],
    }
    for i in 1..=n {
        for j in 1..=n {
            if a[i - 1][j - 1] != b[i - 1][j - 1] {
                println!("{i} {j}");
            }
        }
    }
}
