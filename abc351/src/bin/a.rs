use proconio::input;

fn main() {
    input! {
        a: [i32; 9],
        b: [i32; 8],
    }
    println!(
        "{}",
        a.into_iter().sum::<i32>() - b.into_iter().sum::<i32>() + 1
    );
}
