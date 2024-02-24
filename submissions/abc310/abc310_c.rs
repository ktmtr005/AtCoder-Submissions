use im_rc::HashSet;
use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut ball_set = HashSet::new();
    for i in s {
        let i_rev = i.chars().rev().collect::<String>();
        ball_set.insert(i.min(i_rev));
    }
    let ans = ball_set.len();
    println!("{}", ans);
}