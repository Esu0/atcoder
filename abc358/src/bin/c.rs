use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [marker::Bytes; n],
    }
    let a = s
        .into_iter()
        .map(|si| {
            let mut ai = 0u32;
            for (j, sij) in si.into_iter().enumerate() {
                if sij == b'o' {
                    ai |= 1 << j;
                }
            }
            ai
        })
        .collect::<Vec<_>>();
    // a.iter().copied().for_each(|ai| eprintln!("{:01$b}", ai, m));
    let mut min_count = u32::MAX;
    for i in 0..1u32 << n {
        let mut map = 0;
        let mut b = 1;
        for &aj in &a {
            if b & i != 0 {
                map |= aj;
            }
            b <<= 1;
        }
        if map == (1 << m) - 1 {
            min_count = min_count.min(i.count_ones());
        }
    }
    println!("{min_count}");
}
