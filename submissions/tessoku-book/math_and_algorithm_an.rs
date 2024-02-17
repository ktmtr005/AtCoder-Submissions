use std::collections::VecDeque;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut g = vec![vec![]; n + 1];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut dist = vec![-1; n + 1];
    let mut que = VecDeque::new();
    que.push_back(1);
    dist[1] = 0;
    while que.len() > 0 {
        let pos = que.pop_front().unwrap();
        for &v in &g[pos] {
            let to = v;
            if dist[to] == -1 {
                dist[to] = dist[pos] + 1;
                que.push_back(to);
            }
        }
    }
    for i in 1..dist.len() {
        println!("{}", dist[i]);
    }
}