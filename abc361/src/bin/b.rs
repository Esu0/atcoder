use proconio::input;

fn intersec(a: u32, b: u32, c: u32, d: u32) -> bool {
    !(b <= c || d <= a)
}

fn main() {
    input! {
        a: [u32; 6],
        b: [u32; 6],
    }

    if (0..3).all(|i| intersec(a[i], a[i + 3], b[i], b[i + 3])) {
        println!("Yes");
    } else {
        println!("No");
    }
}
