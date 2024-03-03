use std::collections::BTreeMap;
use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut r: [u64; n],
        query: [u64; q],
    }
    let r_cumsum = {
        r.sort();
        r.iter().scan(0, |cum, x| {
            *cum += x;
            Some(*cum)
        })
        .enumerate()
        .map(|(i, v)| (v, i+1))
        .collect::<BTreeMap<u64, usize>>()
    };
    for x in query {
        let ans = match r_cumsum.range(..=x).next_back() {
            Some((_i, v)) => *v,
            None => 0,
        };
        println!("{}", ans);
    }
}