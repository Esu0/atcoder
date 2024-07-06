use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: u32,
        mut a: [u32; n],
    }

    a.insert(k, x);
    let mut ans_iter = a.iter().copied();
    if let Some(ans) = ans_iter.next() {
        print!("{}", ans);
    }
    for a in ans_iter {
        print!(" {}", a);
    }
    println!();
}
