use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u64; n],
    }

    let mut ans = vec![0; n + 1];
    let mut stack = vec![(0, u64::MAX)];
    for i in 0..n {
        while stack.last().unwrap().1 < h[i] {
            stack.pop();
        }
        let &(last_i, _) = stack.last().unwrap();
        ans[i + 1] = (i + 1 - last_i) as u64 * h[i] + ans[last_i];
        stack.push((i + 1, h[i]));
    }
    // eprintln!("{:?}", ans);
    let mut ans_iter = ans.iter().skip(1);
    if let Some(&a) = ans_iter.next() {
        print!("{}", a + 1);
    }
    for &a in ans_iter {
        print!(" {}", a + 1);
    }
    println!();
}
