use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let ans = solve(a, b);
    if ans == true {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
fn solve(a: i32, b: i32) -> bool{
    let mut ans = false;
    for i in a..=b {
        if 100 % i == 0 {
            ans = true;
        }
    }
    ans
}