use std::{cmp::Reverse, collections::BinaryHeap};
use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        abx: [(u64, u64, usize); n-1],
    }
    let mut g = vec![Vec::new(); n];
    for (i, &(a, b ,x)) in abx.iter().enumerate() {
        g[i].push((i+1, a));
        g[i].push((x-1, b));
    }
    let mut done_node = vec![false; n];
    const INF: u64 = 200_000_000_000_000_000;
    let mut cur = vec![INF; n];
    cur[0] = 0;
    let mut q = BinaryHeap::new();
    q.push(Reverse((cur[0], 0usize)));
    loop {
        let pos = match q.pop() {
            Some(Reverse((_, i))) => i,
            None => break,
        };
        if done_node[pos] == true {
            continue;
        }
        done_node[pos] = true;
        for i in 0..g[pos].len() {
            let (next, cost) = g[pos][i];
            if cur[next] > cur[pos] + cost {
                cur[next] = cur[pos] + cost;
                q.push(Reverse((cur[next], next)));
            }
        }
    }
    let ans = *cur.iter().next_back().unwrap();
    println!("{}", ans);
}