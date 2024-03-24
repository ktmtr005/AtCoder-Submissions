use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let ans = solve(&s);
    println!("{ans}");
}
fn solve(s: &[char]) -> usize {
    s.iter().fold(0, |acc, &x| match x {
        'v' => acc + 1,
        'w' => acc + 2,
        _ => unreachable!(),
    })
}