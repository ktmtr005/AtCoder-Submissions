use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
        s: [String; n],
    }
    match solve(&s) {
        true => println!("Yes"),
        false => println!("No"),
    }
}
fn is_parindrome(s: &String) -> bool {
    let mut ans = true;
    let s_chars = s.as_str().chars().collect::<Vec<char>>();
    for i in 0..s_chars.len() {
        if s_chars[i] != s_chars[s_chars.len() - 1 - i] {
            ans = false;
        }
    }
    ans
}
fn solve(s: &Vec<String>) -> bool {
    let mut ans = false;
    let n = s.len();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let s_concat = format!("{}{}", s[i], s[j]);
            if is_parindrome(&s_concat) == true {
                ans = true;
            }
        }
    }
    ans
}