use proconio::input;

fn main() {
    input! {
        a: u128,
        b: u128,
        c: u128,
        d: u128,
        e: u128,
        f: u128,
    }

    let modulo = 998244353;
    let abc = a * b % modulo * c % modulo;
    let def = d * e % modulo * f % modulo;
    println!("{}", (abc + modulo - def) % modulo);
}
