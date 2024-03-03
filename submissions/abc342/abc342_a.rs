use std::collections::HashMap;
use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut uniq = HashMap::new();
    for i in s.clone() {
        uniq.entry(i).or_insert(0);
        uniq.insert(i, uniq.get(&i).unwrap() + 1);
    }
    for (k, v) in uniq {
        if v == 1 {
            let pos = s.iter().position(|&x| x == k).unwrap();
            println!("{}", pos + 1);
        }
    }
}