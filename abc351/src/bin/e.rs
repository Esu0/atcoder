use proconio::input;

fn partial(p: &[i64]) -> i64 {
    let mut l = 0usize;
    let mut r = p.len().saturating_sub(1);
    let mut sum = 0;
    while l < r {
        sum += (p[r] - p[l]) * (r - l) as i64;
        l += 1;
        r -= 1;
    }
    sum
}

fn main() {
    input! {
        n: usize,
        p: [(u32, u32); n]
    }
    let mut even_x = Vec::new();
    let mut even_y = Vec::new();
    let mut odd_x = Vec::new();
    let mut odd_y = Vec::new();
    for (xi, yi) in p {
        if (xi + yi) % 2 == 0 {
            even_x.push((xi + yi) as i64 / 2);
            even_y.push((yi as i64 - xi as i64) / 2);
        } else {
            odd_x.push((xi + yi - 1) as i64 / 2);
            odd_y.push((yi as i64 - xi as i64 + 1) / 2);
        }
    }
    even_x.sort_unstable();
    even_y.sort_unstable();
    odd_x.sort_unstable();
    odd_y.sort_unstable();
    eprintln!("{even_x:?}");
    println!(
        "{}",
        partial(&even_x) + partial(&even_y) + partial(&odd_x) + partial(&odd_y)
    );
}
