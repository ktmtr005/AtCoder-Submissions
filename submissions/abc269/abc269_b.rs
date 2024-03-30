use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: [String; 10],
    }
    let (a, b, c, d) = solve(s);
    println!("{a} {b}\n{c} {d}");
}
fn solve(s: Vec<String>) -> (usize, usize, usize, usize) {
    let a = s.iter().position(|x| x.contains('#')).unwrap();
    let b = s.iter().rposition(|x| x.contains('#')).unwrap();
    let c = s[a].find('#').unwrap();
    let d = s[a].rfind('#').unwrap();
    (a + 1, b + 1, c + 1, d + 1)
}