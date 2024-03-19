use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let ans = solve(s);
    println!("{ans}");
}
fn solve(s: Vec<char>) -> usize {
    let mut ret = s.len();
    let mut zero_cnt = 1;
    let mut s_iter_peekable = s.iter().peekable();
    while let Some(&c) = s_iter_peekable.next() {
        if c == '0' && s_iter_peekable.peek() == Some(&&'0') {
            zero_cnt += 1;
        } else if c == '0' {
            ret -= zero_cnt / 2;