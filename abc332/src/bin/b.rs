use proconio::input;

fn main() {
    input! {
        k: i32,
        g: i32,
        m: i32,
    }
    let mut g_val = 0;
    let mut m_val = 0;
    for _ in 0..k {
        if g_val == g {
            g_val = 0;
        } else if m_val == 0 {
            m_val = m;
        } else {
            g_val += m_val;
            if g_val > g {
                m_val = g_val - g;
                g_val = g;
            } else {
                m_val = 0;
            }
        }
    }
    println!("{} {}", g_val, m_val)
}
