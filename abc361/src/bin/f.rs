use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    let mut rt = [0; 65];
    for i in 2..64 {
        let mut rti = (n as f64).powf(1.0 / i as f64) as u64;
        // println!("{rti}");
        while rti.checked_pow(i).is_some_and(|x| x < n) {
            rti += 1;
        }
        while rti.checked_pow(i).map_or(true, |x| x > n) {
            rti -= 1;
        }
        rt[i as usize] = rti;
    }
    eprintln!("{rt:?}");
    let primes = [2usize, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59];
    let mut ans = 0;
    
    eprintln!("{}", rt.iter().sum::<u64>());
}
