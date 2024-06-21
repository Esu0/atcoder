use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    print!("2");
    for i in 3..=n {
        let mut is_prime = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            print!(" {}", i);
        }
    }
}
