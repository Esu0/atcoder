use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut m = n;
    while m % 2 == 0 {
        m /= 2;
        print!("2 ");
    }
    let mut i = 3;
    while i * i <= m {
        while m % i == 0 {
            m /= i;
            print!("{} ", i);
        }
        i += 2;
    }
    if m != 1 {
        print!("{}", m);
    }
}
