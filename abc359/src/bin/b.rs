use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n * 2],
    }

    let ans = (1..=n)
        .filter(|&i| {
            a.get(a.iter().position(|&x| x == i).unwrap() + 2)
                .is_some_and(|&aj| aj == i)
        })
        .count();

    println!("{}", ans);
}
