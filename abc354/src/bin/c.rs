use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ac: [(u32, u32); n],
    }
    let mut aci = ac.into_iter().enumerate().collect::<Vec<_>>();
    aci.sort_unstable_by_key(|&(_, (a, _))| Reverse(a));
    let mut c_min = u32::MAX;
    let mut ans = Vec::new();
    for &(i, (_, c)) in &aci {
        if c <= c_min {
            ans.push(i + 1);
            c_min = c;
        }
    }
    ans.sort_unstable();
    println!("{}", ans.len());
    let mut ans_iter = ans.iter().copied();
    if let Some(a) = ans_iter.next() {
        print!("{}", a);
    }
    for a in ans_iter {
        print!(" {}", a);
    }
    println!()
}
