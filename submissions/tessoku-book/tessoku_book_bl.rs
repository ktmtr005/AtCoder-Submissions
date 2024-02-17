use std::collections::BTreeSet;
use amplify::confinement::Collection;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m],
    }
    let mut g = vec![vec![]; n + 1];
    for (a, b, c) in abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    const INF: usize = 2_000_000_000;
    let mut cur = vec![INF; n + 1];
    let mut kakutei = vec![false; n + 1];
    let mut que = BTreeSet::new();
    cur[1] = 0;
    que.push((cur[1], 1));
    while que.len() > 0 {
        let pos = que.first().unwrap().1;
        que.pop_first();
        if kakutei[pos] == true {
            continue;
        }
        kakutei[pos] = true;
        for &(v, c) in &g[pos] {
            let nex = v;
            let cost = c;
            if cur[nex] > cur[pos] + cost {
                cur[nex] = cur[pos] + cost;
                que.push((cur[nex], nex));
            }
        }
    }
    for i in 1..g.len() {
        if cur[i] == INF {
            println!("-1");
        }
        else {
            println!("{}", cur[i]);
        }
    }
}