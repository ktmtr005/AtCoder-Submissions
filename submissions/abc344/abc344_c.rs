use std::collections::HashSet;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        l: usize,
        c: [usize; l],
        q: usize,
        x: [usize; q],
    }
    let ans = solve(a, b, c, x);
    println!("{}", ans.join("\n"));
}
fn solve<'a>(a: Vec<usize>, b: Vec<usize>, c: Vec<usize>, x: Vec<usize>) -> Vec<&'a str> {
    let mut set = HashSet::new();
    for &va in &a {
        for &vb in &b {
            for &vc in &c {
                set.insert(va + vb + vc);
            }
        }
    }
    let mut ret: Vec<&'a str> = Vec::new();
    for v in x {
        match set.get(&v) {
            Some(_i) => ret.push("Yes"),
            None => ret.push("No"),
        }
    }
    ret
}