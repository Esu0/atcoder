use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }

    let count: usize = (1..=n)
        .filter(|i| *i / 10 == 0 || *i / 10 == *i % 10)
        .map(|i| {
            (1..=d[i - 1])
                .filter(|j| i % 10 == *j % 10 && (j / 10 == 0 || j / 10 == i % 10))
                .count()
        })
        // .inspect(|c| println!("{c}"))
        .sum();
    
    println!("{count}");
}
