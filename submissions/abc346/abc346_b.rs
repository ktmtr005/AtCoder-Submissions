use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        w: usize,
        b: usize,
    }
    let ans = solve(w, b);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(w: usize, b: usize) -> bool {
    let mut s = String::new();
    for _i in 0..50 {
        s.push_str("wbwbwwbwbwbw");
    }
    let s = s.chars().collect::<Vec<char>>();
    let s = s.windows(w + b);
    for part in s {
        let w_cnt = part.iter().filter(|&&c| c == 'w').count();
        let b_cnt = part.iter().filter(|&&c| c == 'b').count();
        if w_cnt == w && b_cnt == b {
            return true;
        }
    }
    false
}