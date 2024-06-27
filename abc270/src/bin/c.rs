use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        uv: [(usize, usize); n - 1],
    }
    let mut adj_list = vec![vec![]; n];
    for &(u, v) in &uv {
        adj_list[u - 1].push(v - 1);
        adj_list[v - 1].push(u - 1);
    }

    let mut stack = vec![x - 1];
    let mut prev = vec![usize::MAX; n];
    'dfs: while let Some(node) = stack.pop() {
        for &next in &adj_list[node] {
            if prev[next] < n {
                continue;
            }
            prev[next] = node;
            if next == y - 1 {
                break 'dfs;
            }
            stack.push(next);
        }
    }
    eprintln!("{:?}", prev);
    stack.clear();
    let mut n = y - 1;
    while n != x - 1 {
        stack.push(n);
        n = prev[n];
    }
    print!("{x}");
    for &n in stack.iter().rev() {
        print!(" {}", n + 1);
    }
    println!();
}
