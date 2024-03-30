use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        red: (usize, usize),
        blue: (usize, usize),
    }
    let ans = solve(red, blue);
    println!("{ans}");
}
fn solve((l1, r1): (usize, usize), (l2, r2): (usize, usize)) -> usize {
    r1.min(r2).saturating_sub(l1.max(l2))
}