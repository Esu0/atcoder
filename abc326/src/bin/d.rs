use proconio::{input, marker::Bytes};
use itertools::Itertools;

// struct PermutationIter {
//     n: usize,
//     r: usize,
//     buf: Vec<usize>,
//     stack: Vec<(usize, usize)>,
// }

fn permutation<T, F: FnMut(&[T])>(n: usize, r: usize, k: usize, v: &mut [T], mut f: F) {
    if k == r {
        f(&v);
        return;
    }
    for i in k..n {
        v.swap(i, k);
        permutation(n, r, k + 1, v, &mut f);
        v.swap(i, k);
    }
}
// impl PermutationIter {
//     fn new(n: usize, r: usize) -> Self {
//         let buf: Vec<usize> = (1..=n).collect();
//         Self { n, r, buf, stack: vec![(0, 0)]}
//     }
// }

// impl Iterator for PermutationIter {
//     type Item = Vec<u32>;
//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some((k, mut i)) = self.stack.pop() {
//             self.buf.swap(i, k);
//             if i < self.n {
//                 i += 1;
//                 self.buf.swap(i, k);
//                 self.stack.push((k, i));

//             }
//         }
//         None
//     }
// }
fn main() {
    input! {
        n: usize,
        r: Bytes,
        c: Bytes,
    }
    let mut buf: Vec<usize> = (1..=n).collect();
    permutation(n, n, 0, &mut buf, |a| {
        let mut buf: Vec<usize> = (1..=n).collect();
        permutation(n, n, 0, &mut buf, |b| {
            let mut buf: Vec<usize> = (1..=n).collect();
            permutation(n, n, 0, &mut buf, |c| {
                if a.iter().zip(b.iter().zip(c)).all(|(a, (b, c))| a != b && b != c && c != a) {
                    
                }
            })
        })
    });
}
