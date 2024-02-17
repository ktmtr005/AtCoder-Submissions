use std::collections::BTreeSet;
use proconio::{input, fastout};
#[fastout]
fn main() {
    input!{
        n: usize,
        d: [usize; n],
    }
    let kagamimochi_set: BTreeSet<usize> = BTreeSet::from_iter(d.iter().cloned());
    println!("{}", kagamimochi_set.len());
}