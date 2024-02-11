use proconio::{fastout, input};
use std::cmp::{min, max};
#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n - 1],
    }
    println!("{}", solve(x, &a));
}
fn solve(x: usize, a: &Vec<usize>) -> i32 {
    let mut ans = -1;
    for i in 0..=100usize {
        let mut score_sum = a.iter().sum::<usize>();
        let mut max_score = *a.iter().max().unwrap();
        let mut min_score = *a.iter().min().unwrap();
        score_sum += i;
        max_score = max(max_score, i);
        min_score = min(min_score, i);
        let final_result = score_sum - max_score - min_score;
        if final_result >= x {
            ans = i as i32;
            break;
        }
    }
    ans
}