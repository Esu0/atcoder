use proconio::{input, marker};

const MODULO: u64 = 1_000_000_007;

fn main() {
    input! {
        _n: usize,
        mut s: marker::Bytes,
    }
    s.iter_mut().skip(1).step_by(2).for_each(|c| {
        if *c == b'A' {
            *c = b'B';
        } else {
            *c = b'A';
        }
    });
    // eprintln!("{}", std::str::from_utf8(&s).unwrap());
    let mut count = 1u64;
    let mut ans = 1;
    for w in s.windows(2) {
        let &[a, b] = w else {
            unreachable!()
        };
        if a == b {
            count += 1;
        } else {
            ans = ans * ((count - 1) / 2 + 1) % MODULO;
            count = 1;
        }
    }
    ans = ans * ((count - 1) / 2 + 1) % MODULO;
    println!("{ans}");
}
