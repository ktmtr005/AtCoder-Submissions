use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        _n: usize,
        mut s: Chars,
    }
    let ans = solve(&mut s);
    println!("{ans}");
}
fn solve(s: &mut [char]) -> String {
    let mut enclosed = false;
    for c in &mut *s {
        if *c == '"' {
            enclosed = !enclosed;
        }
        if *c == ',' && !enclosed {
            *c = '.';
        }
    }
    s.iter()
        .map(|&x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
}