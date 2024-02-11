use std::collections::HashMap;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
        m: i32,
        c: [String; n],
        d: [String; m],
        p: [i32; m + 1],
    }
    println!("{}", solve(&c, &d, &p));
}
fn solve(c: &Vec<String>, d: &Vec<String>, p: &Vec<i32>) -> i32 {
    let mut price_map = HashMap::new();
    for (i, v) in d.iter().enumerate() {
        price_map.insert(v, p[i + 1]);
    }
    let mut ans = 0;
    for i in c {
        ans += match price_map.get(i) {
            Some(&i) => i,
            None => p[0],
        }
    }
    ans
}