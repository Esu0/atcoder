use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i32,
        r: i32,
        a: [i32; n],
    }

    let mut x = a.iter().map(|&ai| {
        if ai <= l {
            l
        } else if ai >= r {
            r
        } else {
            ai
        }
    });
    if let Some(x1) = x.next() {
        print!("{x1}");
    }
    for xi in x {
        print!(" {xi}");
    }
    println!();
}
