use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    println!("{}", a.into_iter().sum::<u32>() % 100);
}
