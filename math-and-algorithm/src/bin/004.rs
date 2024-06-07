use proconio::input;

fn main() {
    input! {
        a: [u32; 3],
    }

    println!("{}", a.into_iter().product::<u32>());
}
