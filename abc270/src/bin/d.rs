use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    }
    // Takahashi
    let mut dp1 = vec![(0usize, 0usize); n + 1];
    // Aoki
    let mut dp2 = vec![(0usize, 0usize); n + 1];

    for i in 1..=n {
        let mut max1 = 0;
        let mut max2 = 0;
        for &ak in &a {
            if i >= ak {
                let mut next = dp2[i - ak];
                next.0 += ak;
                if next.0 > max1 {
                    max1 = next.0;
                    dp1[i] = next;
                }

                let mut next = dp1[i - ak];
                next.1 += ak;
                if next.1 > max2 {
                    max2 = next.1;
                    dp2[i] = next;
                }
            }
        }
    }

    println!("{}", dp1[n].0);
}
