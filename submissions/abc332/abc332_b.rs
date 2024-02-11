// -*- coding:utf-8-unix -*-
// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a
fn main() {
    proconio::input! {
        k: i32,
        g: i32,
        m: i32,
    }
    let mut g_wat = 0;
    let mut m_wat = 0;
    for _i in 0..k {
        if g_wat == g {
            g_wat = 0;
        }
        else if m_wat == 0 {
            m_wat = m;
        }
        else {
            if g - g_wat < m_wat {
                m_wat -= g - g_wat;
                g_wat = g;
            }
            else {
                g_wat += m_wat;
                m_wat = 0;
            }
        }
    }
    println!("{} {}", g_wat, m_wat);
}