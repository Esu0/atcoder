pub fn ans_a(x: u32, s: Vec<u32>) -> u32 {
    s.into_iter().filter(|si| si <= &x).sum()
}