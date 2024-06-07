use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let width = c - a;
    let height = d - b;
    let mut s = 0;
    s += (width / 4) * (height / 2) * 8;
    let w_rem = width % 4;
    let h_rem = height % 2;

    let rec_u = d - h_rem;
    s += (width / 4) * h_rem * 4;

    let rec_r = c - w_rem;
    for x in rec_r..c {
        if x.rem_euclid(4) == 0 || x.rem_euclid(4) == 1 {
            s += height / 2 * 3;
        } else {
            s += height / 2;
        }
    }
    let pat = [
        [2, 1, 0, 1], // y % 2 == 0
        [1, 2, 1, 0], // y % 2 == 1
    ];
    for i in rec_r..c {
        for j in rec_u..d {
            s += pat[j.rem_euclid(2) as usize][i.rem_euclid(4) as usize];
        }
    }
    println!("{s}");
}
