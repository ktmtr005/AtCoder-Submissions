use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        k: usize,
    }
    println!("{}", solve(k));
}
fn solve(k: usize) -> i64 {
    let mut ans = Vec::new();
    for i in 2..(1 << 10) {
        let mut x = 0i64;
        for j in (0..=9).rev() {
            if i & (1 << j) != 0 {
                x *= 10;
                x += j;
            }
        }
        ans.push(x);
    }
    ans.sort();
    ans[k - 1]
}