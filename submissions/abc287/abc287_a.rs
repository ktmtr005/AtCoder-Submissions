use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let ans = solve(s);
    println!("{}", if ans == true {"Yes"} else {"No"});
}
fn solve(s: Vec<String>) -> bool {
    let (mut y, mut n) = (0, 0);
    for i in s {
        if i == "For" {
            y += 1;
        }
        else {
            n += 1;
        }
    }
    y > n
}