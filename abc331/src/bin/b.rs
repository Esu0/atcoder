use proconio::input;

fn main() {
    input! {
        n: i32,
        s: i32,
        m: i32,
        l: i32,
    }
    println!(
        "{}",
        (0..=17)
            .flat_map(|si| (0..=13).map(move |mi| (0..=9).map(move |li| (si, mi, li))))
            .flatten()
            .filter(|&(si, mi, li)| si * 6 + mi * 8 + li * 12 >= n)
            .map(|(si, mi, li)| si * s + mi * m + li * l)
            .min()
            .unwrap()
    );
}
