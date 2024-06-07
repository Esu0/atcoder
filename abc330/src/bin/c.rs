use std::cmp;

use proconio::input;

fn main() {
    input! {
        d: i64,
    }

    let mut min = i64::MAX;
    for x in 0i64.. {
        let dif = d - x * x;
        if dif <= 0 {
            break;
        }
        let rt = (dif as f64).sqrt();
        let y1 = rt.floor() as i64;
        let y2 = rt.ceil() as i64;
        min = cmp::min(min, (dif - y1 * y1).abs());
        min = cmp::min(min, (dif - y2 * y2).abs());
    }
    println!("{min}");
}
