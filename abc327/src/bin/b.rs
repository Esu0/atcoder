use proconio::input;

fn main() {
    input! {
        b: u64,
    }
    let mut a = 1u64;
    while let Some(aa) = a.checked_pow(a as u32) {
        if aa == b {
            println!("{a}");
            return;
        }
        a += 1;
    }
    println!("-1");
}
