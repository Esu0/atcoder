use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let s = s.into_iter().map(|si| si.into_bytes()).collect::<Vec<_>>();
    let row_o_count = s.iter().map(|si| si.iter().filter(|&&c| c == b'o').count()).collect::<Vec<_>>();
    let column_o_count = (0..n).map(|i| s.iter().filter(|&sj| sj[i] == b'o').count()).collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] != b'o' {
                continue;
            }
            ans += (row_o_count[i] - 1) * (column_o_count[j] - 1);
        }
    }
    println!("{ans}");
}
