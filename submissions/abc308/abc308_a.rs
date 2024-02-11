use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: [i32; 8],
    }
    match solve(&s) {
        true => println!("Yes"),
        false => println!("No"),
    }
}
fn solve(s: &Vec<i32>) -> bool {
    let mut ans = true;
    for i in 1..s.len() {
        if s[i - 1] > s[i] {
            ans = false;
        }
    }
    for &i in s {
        if i < 100 || 675 < i {
            ans = false;
        }
        if i % 25 != 0 {
            ans = false;
        }
    }
    ans
}