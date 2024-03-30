use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let ans = solve(s);
    match ans {
        Some(c) => println!("{c}"),
        None => println!("-1"),
    }
}
fn solve(s: Vec<char>) -> Option<char> {
    let ascii_lower: Vec<char> = (0..26).map(|x| (b'a' + x) as char).collect();
    ascii_lower
        .into_iter()
        .find(|&c| s.iter().filter(|&&x| x == c).count() == 1)
}