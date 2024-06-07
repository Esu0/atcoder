use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i32,
        a: [[i32; n]; n],
    }
    let mut new_a = a.clone();
    for k in [0, d] {
        let mut cost = 0;
        for i in 0..n {
            for j in 0..n {
                let aij = a[i][j];
                let rem = aij.rem_euclid(2 * d);
                let diff = if (i + j) % 2 == 0 {
                    std::cmp::min_by_key(k - rem, 2 * d - rem + k, |&x| x.abs())
                } else {
                    std::cmp::min_by_key(
                        (k + d) % (2 * d) - rem,
                        (k + d) % (2 * d) + 2 * d - rem,
                        |&x| x.abs(),
                    )
                };
                new_a[i][j] = aij + diff;
                if (i + j) % 2 == 0 {
                    assert_eq!(new_a[i][j].rem_euclid(2 * d), k)
                } else {
                    assert_eq!(new_a[i][j].rem_euclid(2 * d), (k + d) % (2 * d));
                }
                cost += diff.abs();
            }
        }
        if cost * 2 <= d * n as i32 * n as i32 {
            eprintln!("cost = {}", cost);
            for i in 0..n {
                print!("{}", new_a[i][0]);
                for j in 1..n {
                    print!(" {}", new_a[i][j]);
                }
                println!();
            }
            return;
        }
    }
    panic!();
}
