use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        t: u32,
        a: [u32; n - 1],
        xy: [(usize, u32); m],
    }

    let mut remaining = t;
    let mut i = 1;
    let mut j = 0;
    loop {
        if i == n {
            println!("Yes");
            return;
        }
        if xy.get(j).is_some_and(|(x, _)| i == *x) {
            remaining += xy[j].1;
            j += 1;
        }
        if let Some(next_remaining) = remaining.checked_sub(a[i - 1]) {
            if next_remaining != 0 {
                remaining = next_remaining;
                i += 1;
                continue;
            }
        }
        println!("No");
        return;
    }
}
