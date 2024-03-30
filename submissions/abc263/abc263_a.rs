use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        card: [usize; 5],
    }
    let ans = solve(card);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(card: Vec<usize>) -> bool {
    let mut cnt = [0; 14];
    for i in card {
        cnt[i] += 1;
    }
    cnt.contains(&0) && cnt.contains(&2) && cnt.contains(&3)
}