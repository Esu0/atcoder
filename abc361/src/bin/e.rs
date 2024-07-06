use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [(usize, usize, u64); n - 1],
    }
    let mut adj_list = vec![vec![]; n];
    for &(a, b, c) in &abc {
        adj_list[a - 1].push((b - 1, c));
        adj_list[b - 1].push((a - 1, c));
    }

    let mut stack = vec![0usize];
    let mut dist = vec![u64::MAX; n];
    dist[0] = 0;
    while let Some(node) = stack.pop() {
        let d = dist[node];
        for &(next, d2) in &adj_list[node] {
            if dist[next] != u64::MAX {
                continue;
            }
            dist[next] = d + d2;
            stack.push(next);
        }
    }
    let max_dist_node = dist.iter().enumerate().max_by_key(|&(_, &d)| d).unwrap().0;

    stack.clear();
    dist.fill(u64::MAX);
    dist[max_dist_node] = 0;
    stack.push(max_dist_node);
    while let Some(node) = stack.pop() {
        let d = dist[node];
        for &(next, d2) in &adj_list[node] {
            if dist[next] != u64::MAX {
                continue;
            }
            dist[next] = d + d2;
            stack.push(next);
        }
    }

    let max_dist = dist.iter().copied().max().unwrap();
    eprintln!("{max_dist}");
    let dist_sum = abc.iter().map(|&(_, _, c)| c).sum::<u64>();
    println!("{}", dist_sum * 2 - max_dist);
}
