use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u32,
        t: [u32; n],
    }
    let mut time = 0;
    for &ti in &t {
        time = time.max(ti) + a;
        println!("{time}");
    }
}
