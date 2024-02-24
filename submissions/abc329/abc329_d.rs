use std::{cmp::Reverse, collections::BTreeSet};
use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m]
    }
    let mut votes = vec![0usize; n + 1];
    let mut quick_report = BTreeSet::new(); // (votes, Reverse(candidate))
    for i in a {
        votes[i] += 1;
        quick_report.insert((votes[i], Reverse(i)));
        let winner = match quick_report.iter().last() {
            Some(&(_, Reverse(i))) => i,
            None => unreachable!(),
        };
        println!("{}", winner);
    }
}