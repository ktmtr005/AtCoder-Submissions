use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let ans = solve(n, a)
        .iter()
        .map(std::string::ToString::to_string)
        .join("\n");
    println!("{ans}");
}
fn solve(n: usize, a: Vec<usize>) -> Vec<usize> {
    let mut ans = vec![0; 2 * n + 1];
    for (i, &a_i) in a.iter().enumerate() {
        ans[2 * i + 1] = ans[a_i] + 1;
        ans[2 * i + 2] = ans[a_i] + 1;
    }
    ans
}