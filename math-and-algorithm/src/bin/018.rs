use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut count = [0u32; 4];
    for &ai in &a {
        count[(ai / 100 - 1) as usize] += 1;
    }
    let ans = count[0] * count[3] + count[1] * count[2];
    println!("{}", ans);
}