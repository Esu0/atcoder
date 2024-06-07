use proconio::input;

fn main() {
    input! {
        n: usize,
        l: u32,
        a: [u32; n],
    }

    println!("{}", a.iter().copied().filter(|&s| s >= l).count());
}
