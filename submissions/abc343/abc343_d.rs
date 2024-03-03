use std::collections::HashMap;
use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        ab: [(usize, u64); t],
    }
    let mut ans = 1;
    let mut scores = vec![0u64; n];
    let mut map = HashMap::new();
    map.entry(0).or_insert(n);
    for (a, b) in ab {
        *map.entry(scores[a-1]).or_insert(0)-= 1;
        if let Some(&i) = map.get(&scores[a-1]) {
            if i == 0 {
               ans -= 1; 
            }
        }
        scores[a-1] += b;
        *map.entry(scores[a-1]).or_insert(0) += 1;
        if let Some(&i) = map.get(&scores[a-1]) {
            if i == 1 {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}