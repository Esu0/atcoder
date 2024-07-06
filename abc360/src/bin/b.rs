use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
        t: marker::Bytes,
    }
    for w in 1..s.len() {
        for c in 0..w {
            let t1 = s.iter().skip(c).step_by(w).copied().collect::<Vec<_>>();
            eprintln!("{}", std::str::from_utf8(&t1).unwrap());
            if t == t1 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
