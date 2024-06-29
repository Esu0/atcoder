use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u64; n],
        mut b: [u64; m],
    }

    a.sort_unstable();
    b.sort_unstable();
    let mut i = 0;
    let mut ans = 0;
    'outer: for &bj in &b {
        while let Some(&ai) = a.get(i) {
            if ai >= bj {
                ans += ai;
                i += 1;
                continue 'outer;
            }
            i += 1;
        }
        println!("-1");
        return;
    }
    println!("{ans}");
}
