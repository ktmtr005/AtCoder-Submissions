use std::collections::BTreeSet;
use std::string::ToString;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m],
    }
    let (ans_a, ans_b) = solve(n, m, a, b);
    println!(
        "{}\n{}",
        ans_a
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" "),
        ans_b
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}
fn solve(n: usize, m: usize, a: Vec<u32>, b: Vec<u32>) -> (Vec<usize>, Vec<usize>) {
    let a: Vec<(u32, char)> = a.iter().map(|&n| (n, 'a')).collect();
    let b: Vec<(u32, char)> = b.iter().map(|&n| (n, 'b')).collect();
    let c_with_nth = [a, b]
        .concat()
        .into_iter()
        .collect::<BTreeSet<(u32, char)>>()
        .into_iter()
        .enumerate();
    let mut a_nth = Vec::with_capacity(n);
    let mut b_nth = Vec::with_capacity(m);
    for (i, (_n, c)) in c_with_nth {
        if c == 'a' {
            a_nth.push(i + 1);
        } else {
            b_nth.push(i + 1);
        }
    }
    (a_nth, b_nth)
}