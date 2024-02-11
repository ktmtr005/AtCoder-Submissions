use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
        q: i32,
        a: [i32; n],
        query: [(usize, usize); q],
    }
    let ans = solve(&a, &query);
    for i in ans {
        println!("{}", i);
    }
}
fn solve(a: &Vec<i32>, query: &Vec<(usize, usize)>) -> Vec<i32> {
    let mut sum = vec![0; a.len() + 1];
    for i in 0..a.len() {
        sum[i + 1] = sum[i] + a[i];
    }
    let mut ans = vec![0_i32; query.len()];
    for i in 0..query.len() {
        ans[i] = sum[query[i].1] - sum[query[i].0 - 1];
    }
    ans
}