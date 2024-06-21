use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    if n == 2 {
        println!("Yes");
        return;
    } else if n % 2 == 0 {
        println!("No");
        return;
    }
    let mut is_prime = true;
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            is_prime = false;
            break;
        }
        i += 2;
    }
    if is_prime {
        println!("Yes");
    } else {
        println!("No");
    }
}