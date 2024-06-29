use proconio::input;

fn main() {
    input! {
        s: [String],
    }
    println!("{}", s.iter().filter(|&s| s == "Takahashi").count());
}
