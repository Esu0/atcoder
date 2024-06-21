use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: proconio::marker::Bytes,
    }

    let t_count = s.iter().filter(|&&c| c == b'T').count();
    let a_count = s.iter().filter(|&&c| c == b'A').count();
    use std::cmp::Ordering::*;
    match t_count.cmp(&a_count) {
        Less => println!("A"),
        Equal => {
            if s.last().unwrap() == &b'A' {
                println!("T");
            } else {
                println!("A");
            }
        }
        Greater => println!("T"),
    }
}
