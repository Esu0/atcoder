use proconio::input;

fn main() {
    input! {
        mut a: u64,
        mut b: u64,
    }

    while a != 0 {
        let t = b % a;
        b = a;
        a = t;
    }
    println!("{}", b);
}