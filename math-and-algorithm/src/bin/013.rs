use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut i = 1;
    while i * i < n {
        if n % i == 0 {
            println!("{}\n{}", i, n / i);
        }
        i += 1;
    }
    if i * i == n {
        println!("{}", i);
    }
}
