use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        s: [u32; n],
    }

    let sum = abc328::ans_a(x, s);
    println!("{sum}");
}
