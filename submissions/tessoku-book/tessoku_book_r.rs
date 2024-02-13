use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    match solve(s, &a) {
        true => println!("Yes"),
        false => println!("No"),
    }
}
fn solve(s: usize, a: &Vec<usize>) -> bool {
    let mut dp = vec![vec![false; s + 1]; a.len() + 1];
    dp[0][0] = true;
    for i in 1..dp.len() {
        for j in 0..dp[0].len() {
            if j < a[i - 1] {
                if dp[i - 1][j] == true {
                    dp[i][j] = true;
                }
            }
            if j >= a[i - 1] {
                if dp[i - 1][j] == true || dp[i - 1][j - a[i - 1]] == true {
                    dp[i][j] = true;
                }
            }
        }
    }
    dp[dp.len() - 1][dp[0].len() - 1]
}