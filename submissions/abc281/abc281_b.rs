use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: String,
    }
    let ans = solve(&s);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(s: &str) -> bool {
    let first = s.chars().next().unwrap().is_alphabetic();
    let last = s.chars().last().unwrap().is_alphabetic();
    if first && last && s.len() == 8 {
        if let Ok(n) = s[1..s.len() - 1].parse::<u32>() {
            if (100000..=999999).contains(&n) {
                return true;
            }
        }
    }
    false
}