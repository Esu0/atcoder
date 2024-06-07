use proconio::input;

fn main() {
    input! {
        h: usize,
    }
    let mut sum = 0;
    for i in 0u32..32 {
        sum += 1 << i;
        if sum > h {
            println!("{}", i + 1);
            return;
        }
    }
}
