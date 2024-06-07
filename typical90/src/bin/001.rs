use proconio::input;

fn binary_search(min: usize, max: usize, mut f: impl FnMut(usize) -> bool) -> usize {
    let mut min = min;
    let mut max = max;
    while max - min > 1 {
        let mid = (max + min) / 2;
        if f(mid) {
            min = mid;
        } else {
            max = mid;
        }
    }
    min
}

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n],
    }
    let solve = |s| {
        let mut i = 0;
        let mut last_cut = 0;
        for _ in 0..k {
            loop {
                if i >= n {
                    return false;
                }
                let next_cut = a[i];
                i += 1;
                if next_cut - last_cut >= s {
                    last_cut = next_cut;
                    break;
                }
            }
        }
        l - last_cut >= s
    };
    let ans = binary_search(1, l / (k + 1) + 1, &solve);
    // eprintln!("{}", solve(2));
    println!("{}", ans);
}
