use proconio::input;

fn main() {
    input! {
        mut sx: i64,
        sy: i64,
        mut tx: i64,
        ty: i64,
    }
    if sy % 2 == 0 {
        sx &= !1;
    } else if sx % 2 == 0 {
        sx -= 1;
    }
    if ty % 2 == 0 {
        tx &= !1;
    } else if tx % 2 == 0 {
        tx -= 1;
    }
    let mut ans = ty.abs_diff(sy);
    let mut diff = tx.abs_diff(sx);
    if diff > ans {
        diff -= ans;
        ans += diff / 2;
    }
    println!("{}", ans);
}
