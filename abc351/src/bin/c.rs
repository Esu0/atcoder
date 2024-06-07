use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut column = Vec::new();
    for ai in a {
        column.push(ai);
        loop {
            let len = column.len();
            if len <= 1 {
                break;
            }
            let r1 = column[len - 1];
            let r2 = column[len - 2];
            if r1 != r2 {
                break;
            }
            column.truncate(len - 1);
            column[len - 2] = r1 + 1;
        }
    }
    println!("{}", column.len());
}
