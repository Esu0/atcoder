use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
    }
    let mut res = vec![];
    for c in s {
        res.push(c);
        if res.ends_with(b"ABC") {
            res.truncate(res.len() - 3);
        }
    }
    if !res.is_empty() {
        println!("{}", String::from_utf8(res).unwrap());
    }
}
