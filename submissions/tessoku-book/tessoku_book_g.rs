use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        d: usize,
        n: i32,
        query: [(usize, usize); n],
    }
    let ans = solve(d, &query);
    for i in 1..ans.len() - 1 {
        println!("{}", ans[i]);
    }
}
fn solve(d: usize, query: &Vec<(usize, usize)>) -> Vec<i32> {
    let mut attendance = vec![0; d + 2];
    for &(l, r) in query {
        attendance[l] += 1;
        attendance[r + 1] -= 1;
    }
    let mut ans = vec![0; d + 2];
    for i in 1..attendance.len() - 1 {
        ans[i] = ans[i - 1] + attendance[i];
    }
    ans
}