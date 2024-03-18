use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let ans = solve(s);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(s: Vec<char>) -> bool {
    let begin = *s.first().unwrap();
    let end = *s.iter().last().unwrap();
    if begin != '<' || end != '>' {
        return false;
    }
    for &c in s.iter().skip(1).take(s.len() - 2) {
        if c != '=' {
            return false;
        }
    }
    true
}