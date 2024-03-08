use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        _n: usize,
        s: String,
    }
    let ans = solve(s);
    println!("{}", ans);
}
fn solve(mut s: String) -> String {
    let mut i = 0;
    while let Some(c) = s.get(i..=i+1) {
        if c == "na" {
            s.insert(i+1, 'y');
        }
        i += 1;
    }
    s
}