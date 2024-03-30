use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let ans = solve(a);
    match ans {
        Some(i) => println!("{i}"),
        None => println!("-1"),
    }
}
fn solve(mut a: Vec<u64>) -> Option<u64> {
    a.sort_by(|a, b| b.cmp(a));
    let mut even = Vec::new();
    let mut odd = Vec::new();
    for &i in &a {
        if i % 2 == 0 {
            even.push(i);
        } else {
            odd.push(i);
        }
    }
    let max_even = even.get(0..2);
    let max_odd = odd.get(0..2);
    match (max_even, max_odd) {
        (None, None) => None,
        (Some(s), None) => Some(s[0] + s[1]),
        (None, Some(s)) => Some(s[0] + s[1]),
        (Some(s1), Some(s2)) => Some((s1[0] + s1[1]).max(s2[0] + s2[1])),
    }
}