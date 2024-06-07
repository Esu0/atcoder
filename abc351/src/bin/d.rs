use proconio::{input, marker::Bytes};

fn can_move(s: &[Vec<u8>], i: usize, j: usize) -> bool {
    j.checked_sub(1).map(|j| s[i][j] != b'#').unwrap_or(true)
        && s[i].get(j + 1).map(|sij| *sij != b'#').unwrap_or(true)
        && i.checked_sub(1).map(|i| s[i][j] != b'#').unwrap_or(true)
        && s.get(i + 1).map(|si| si[j] != b'#').unwrap_or(true)
}
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Bytes; h],
    }
    let mut visited = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if visited[i][j] {
                continue;
            }

            
        }
    }
}
