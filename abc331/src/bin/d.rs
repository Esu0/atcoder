use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [Bytes; n],
        query: [(i64, i64, i64, i64); q],
    }
}
