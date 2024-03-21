use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let ans = solve(s, t);
    println!("{ans}");
}
fn solve(s: Vec<char>, t: Vec<char>) -> usize {
    for (i, (&c1, &c2)) in s.iter().zip(t.iter()).enumerate() {
        if c1 != c2 {
            return i + 1;
        }
    }
    t.len()
}