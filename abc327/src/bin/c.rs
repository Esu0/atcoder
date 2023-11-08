use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a: [[u8; 9]; 9]
    }

    let cmp = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let cond1 = a.iter().all(|row| cmp == row.iter().copied().collect());
    let cond2 = (0..9).all(|i| cmp == (0..9).map(|j| a[j][i]).collect());
    let cond3 = (0..9).step_by(3).all(|i| {
        (0..9).step_by(3).all(|j| {
            cmp == (0..3)
                .flat_map(|k| {
                    (0..3)
                        .map(move |l| (i + k, j + l))
                        .map(|(i1, i2)| a[i1][i2])
                })
                .collect()
        })
    });

    if cond1 && cond2 && cond3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
