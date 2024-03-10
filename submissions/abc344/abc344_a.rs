use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        s: String,
    }
    let ans = solve(s);
    println!("{}", ans);
}
fn solve(s: String) -> String {
    let mut ret = String::new();
    let mut spoil = false;
    for i in s.chars() {
        match i {
            '|' if spoil == false => spoil = true,
            '|' if spoil == true => spoil = false,
            _ if spoil == false => ret.push(i),
            _ => continue,
        }
    }
    ret
}