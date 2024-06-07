use proconio::input;
use smallvec::SmallVec;

fn solve(n: u8) {
    assert!(n <= 20);
    if n % 2 == 1 {
        return;
    }
    let mut stack = SmallVec::<[u8; 20]>::new();
    solve_rec(n, &mut stack, 0);
}
fn solve_rec(n: u8, stack: &mut SmallVec::<[u8; 20]>, left_bracket_count: u8) {
    if stack.len() == n as usize {
        println!("{}", std::str::from_utf8(stack).unwrap());
    }
    if left_bracket_count < n / 2 {
        stack.push(b'(');
        solve_rec(n, stack, left_bracket_count + 1);
        stack.pop();
    }
    if stack.len() as u8 - left_bracket_count < left_bracket_count {
        stack.push(b')');
        solve_rec(n, stack, left_bracket_count);
        stack.pop();
    }
}
fn main() {
    input! {
        n: u8,
    }
    solve(n);
}
