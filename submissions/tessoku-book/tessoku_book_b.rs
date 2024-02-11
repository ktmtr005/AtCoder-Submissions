use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
        x: i32,
        a: [i32; n],
    }
    let ans = solve(x, &a);
    if ans == true {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
fn solve(x: i32, a: &Vec<i32>) -> bool {
    let mut ans = false;
    for i in a {
        if *i == x {
            ans = true;
        }
    }
    ans
}