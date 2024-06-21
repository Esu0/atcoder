use proconio::input;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        let t = b % a;
        b = a;
        a = t;
    }
    b
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let ans = a.into_iter().reduce(gcd).unwrap();
    println!("{}", ans);
}
