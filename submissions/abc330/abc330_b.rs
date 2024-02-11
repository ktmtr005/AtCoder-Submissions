// abc330_b
use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: i32,
        l: i32,
        r: i32,
        a: [i32; n],
    }
    let ans = solve(l, r, &a);
    for i in 0..ans.len() {
        print!("{}", ans[i]);
        if i < ans.len() - 1 {
            print!(" ");
        }
    }
    print!("\n");
}
fn solve(l: i32, r: i32, a: &Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0_i32; a.len()];
    assert!(a.len() == ans.len());
    for i in 0..a.len() {
        if l <= a[i] && a[i] <= r {
            ans[i] = a[i];
        }
        else if (l - a[i]).abs() < (r - a[i]).abs() {
            ans[i] = l;
        }
        else {
            ans[i] = r;
        }
    }
    ans
}