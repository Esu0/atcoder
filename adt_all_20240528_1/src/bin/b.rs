use proconio::input;

fn main() {
    input! {
        n: usize,
        sa: [(String, u32); n],
    }
    let (min, _) = sa.iter().enumerate().min_by_key(|(_, (_, a))| *a).unwrap();
    for i in 0..n {
        println!("{}", sa[(i + min) % n].0);
    }
}
