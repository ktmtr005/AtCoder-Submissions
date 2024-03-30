use std::collections::HashSet;
use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let ans = solve(n, uv);
    println!("{ans}");
}
fn solve(n: usize, uv: Vec<(usize, usize)>) -> usize {
    let mut g = vec![HashSet::new(); n];
    for (u, v) in uv {
        g[u].insert(v);
        g[v].insert(u);
    }
    let mut cnt = 0;
    for (a, a_b) in g.iter().enumerate() {
        for &b in a_b.iter().filter(|&&x| a < x) {
            for &c in g[b].iter().filter(|&&x| b < x) {
                if g[c].get(&a).is_some() {
                    cnt += 1;
                }
            }
        }
    }
    cnt
}