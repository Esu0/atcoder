use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m],
    }
    // (out-degree, in-degree)
    let mut degree = vec![(0u32, 0u32); n];
    let mut adj_list = vec![vec![]; n];
    for (x, y) in xy {
        degree[x - 1].0 += 1;
        degree[y - 1].1 += 1;
        adj_list[x - 1].push(y - 1);
    }

    let mut starts = degree
        .iter()
        .enumerate()
        .filter(|&(_, &(_, indeg))| indeg == 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut depth = 0;
    loop {
        let mut tmp = vec![];
        for &start in &starts {
            for &next in &adj_list[start] {
                degree[next].1 -= 1;
                if degree[next].1 == 0 {
                    tmp.push(next);
                }
            }
        }
        starts = tmp;
        if starts.is_empty() {
            println!("{}", depth);
            return;
        }
        depth += 1;
    }
}
