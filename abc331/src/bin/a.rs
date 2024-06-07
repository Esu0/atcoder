use proconio::input;

fn main() {
    input! {
        lm: i32,
        ld: i32,
        mut y: i32,
        mut m: i32,
        mut d: i32,
    }
    d += 1;
    if d >= ld {
        d -= ld;
        m += 1;
        if m >= lm {
            m -= lm;
            y += 1;
        }
    }
    println!("{y} {m} {d}");
}
