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
    if s[0] == '1' {
        return false;
    }
    let pins = [
        // 各列のピンがすべて倒れている -> true
        s[6] == '0',
        s[3] == '0',
        (s[7] == '0') && (s[1] == '0'),