use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: marker::Bytes,
    }
    let ans = s.split(|&c| c == b'0').map(|x| {
        let count1 = x.iter().filter(|&&c| c == b'1').count();
        let count2 = x.iter().filter(|&&c| c == b'2').count();
        if count1 > m {
            count1 - m + count2
        } else {
            count2
        }
    }).max().unwrap();
    println!("{ans}");
}
