use std::sync::atomic::{AtomicUsize, Ordering::*};

use proconio::input_interactive;

static Q: AtomicUsize = AtomicUsize::new(0);

fn query(a: u8, b: u8) -> bool {
    if Q.load(Relaxed) == 0 {
        println!("info: query limit exceeded");
    } else {
        Q.fetch_sub(1, Relaxed);
    }
    println!("? {} {}", (b'A' + a) as char, (b'A' + b) as char);
    input_interactive! {
        c: char,
    }
    match c {
        '>' => true,
        '<' => false,
        _ => unreachable!(),
    }
}

fn solve(v: &mut [u8], buf: &mut [u8]) {
    let n = v.len();
    match n {
        0 => return,
        1 => {
            buf[0] = v[0];
            return;
        }
        2 => {
            if query(v[0], v[1]) {
                buf[0] = v[1];
                buf[1] = v[0];
            } else {
                buf.copy_from_slice(v);
            }
            return;
        },
        _ => (),
    }
    let m = n / 2;
    let (l, r) = v.split_at_mut(m);
    let (lbuf, rbuf) = buf.split_at_mut(m);
    solve(l, lbuf);
    solve(r, rbuf);
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    loop {
        if query(lbuf[i], rbuf[j]) {
            v[k] = rbuf[j];
            j += 1;
            k += 1;
            if j >= rbuf.len() {
                v[k..].copy_from_slice(&lbuf[i..]);
                break;
            }
        } else {
            v[k] = lbuf[i];
            i += 1;
            k += 1;
            if i >= lbuf.len() {
                v[k..].copy_from_slice(&rbuf[j..]);
                break;
            }
        }
    }
    buf.copy_from_slice(v);
}

fn main() {
    input_interactive! {
        n: usize,
        q: usize,
    }
    Q.store(q, Relaxed);
    let mut v = (0..n as u8).collect::<Vec<_>>();
    let mut buf = vec![0; n];
    solve(&mut v, &mut buf);
    eprintln!("{:?}", buf);
}
