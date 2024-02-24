use std::collections::BTreeSet;
use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    }
    let mut dict = BTreeSet::new();
    for i in 0..k {
        dict.insert(s[i].clone());
    }
    dict.iter().for_each(|x| println!("{x}"));
}