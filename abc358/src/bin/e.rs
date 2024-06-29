use proconio::input;

fn gcd_ext(a: usize, b: usize) -> (usize, isize, isize) {
    assert!(a != 0 || b != 0);
    let mut x_prev = 1;
    let mut x = 0;
    let mut r_prev = a;
    let mut r = b;

    while r != 0 {
        (x_prev, x) = (x, x_prev - x * (r_prev / r) as isize);
        (r_prev, r) = (r, r_prev % r);
    }
    (
        r_prev,
        x_prev,
        (r_prev as isize - a as isize * x_prev) / b as isize,
    )
}

fn minv(a: usize, modulo: usize) -> usize {
    let (_, x, _) = gcd_ext(a, modulo);
    if x.is_negative() {
        (x + modulo as isize) as usize
    } else {
        x as usize
    }
}

struct FactrialMod {
    memo: Vec<usize>,
    memo_inv: Vec<usize>,
    modulo: usize,
}

impl FactrialMod {
    fn new(cap: usize, modulo: usize) -> Self {
        let mut v = 1;
        let memo = std::iter::once(1)
            .chain((1..=cap).map(|i| {
                v = v * i % modulo;
                v
            }))
            .collect::<Vec<_>>();
        Self {
            memo_inv: memo.iter().map(|&v| minv(v, modulo)).collect(),
            memo,
            modulo,
        }
    }

    fn get(&self, n: usize) -> usize {
        self.memo[n]
    }

    fn get_inv(&self, n: usize) -> usize {
        self.memo_inv[n]
    }

    fn combination(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.get(n) * self.get_inv(k) % self.modulo * self.get_inv(n - k) % self.modulo
        }
    }
}

const MOD: usize = 998244353;
fn main() {
    input! {
        k: usize,
    }
    let c = std::array::from_fn::<_, 26, _>(|_| {
        input! {
            c: usize,
        }
        c
    });
    let mut dp = std::array::from_fn::<_, 26, _>(|_| vec![0usize; k + 1]);
    dp[0][..=std::cmp::min(c[0], k)].fill(1);
    let factorial_memo = FactrialMod::new(k + 1, MOD);
    for i in 1..26 {
        dp[i][0] = 1;
        for j in 1..=k {
            for l in (0..=j).rev() {
                let m = j - l;
                if m > c[i] {
                    break;
                }
                dp[i][j] += factorial_memo.combination(j, m) * dp[i - 1][l] % MOD;
            }
            dp[i][j] %= MOD;
        }
    }
    // eprintln!("{dp:?}");
    println!("{}", dp[25].iter().skip(1).sum::<usize>() % MOD);
}
