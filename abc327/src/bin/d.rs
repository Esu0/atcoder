use petgraph::prelude::*;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        a: [usize; m],
        b: [usize; m],
    }

    let mut g: Graph<Option<bool>, (), Undirected, _> =
        Graph::from_edges(a.into_iter().zip(b).map(|(a, b)| (a - 1, b - 1)));
    for i in g.node_indices() {
        if let None = g[i] {
            g[i] = Some(true);
            let mut bfs = Bfs::new(&g, i);
            // println!("{}", i.index());
            while let Some(j) = bfs.next(&g) {
                let mut neighbors = g.neighbors(j).detach();
                while let Some(v) = neighbors.next_node(&g) {
                    let new = !g[j].unwrap();
                    let old = g[v].replace(new);
                    match old {
                        None => continue,
                        Some(o) if o == new => continue,
                        Some(_) => {
                            println!("No");
                            return;
                        }
                    }
                }
            }
        } else {
            continue;
        }
    }
    println!("Yes");
}
