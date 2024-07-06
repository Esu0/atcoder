use proconio::input;

fn gcd_ext(a: u64, b: u64) -> (u64, i64, i64) {
    assert!(a != 0 || b != 0);
    let mut x_prev = 1;
    let mut x = 0;
    let mut r_prev = a;
    let mut r = b;

    while r != 0 {
        (x_prev, x) = (x, x_prev - x * (r_prev / r) as i64);
        (r_prev, r) = (r, r_prev % r);
    }
    (
        r_prev,
        x_prev,
        (r_prev as i64 - a as i64 * x_prev) / b as i64,
    )
}

fn minv(a: u64, modulo: u64) -> u64 {
    let (_, x, _) = gcd_ext(a, modulo);
    if x.is_negative() {
        (x + modulo as i64) as _
    } else {
        x as _
    }
}

fn main() {
    input! {
        n: u64,
        k: usize,
    }
    const MODULO: u64 = 998244353;
    let n_inv = minv(n, MODULO);
    let c1 = (n + 1) * n_inv % MODULO;
    let c2 = (1 + MODULO * 2 - 2 * n_inv) % MODULO;
    let mut x = 1;
    for _ in 0..k {
        x = (c1 + c2 * x) % MODULO;
    }
    println!("{}", x);
}
