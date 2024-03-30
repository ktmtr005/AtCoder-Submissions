use std::collections::HashMap;
use itertools::Itertools;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let ans = solve(s);
    println!("{}", ans.iter().join("\n"));
}
fn solve(s: Vec<String>) -> Vec<String> {
    let mut map = HashMap::new();
    let mut ret = Vec::with_capacity(s.len());
    for s_i in s {
        if map.get(&s_i).is_none() {
            map.insert(s_i.clone(), 0);
            ret.push(s_i);
        } else {
            *map.entry(s_i.clone()).or_insert(0) += 1;
            let s_fmt = format!("{}({})", s_i, map.get(&s_i).unwrap());
            ret.push(s_fmt);
        }
    }
    ret
}