use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let ans = solve(n, &a);
    if ans == true {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
fn solve(n: usize, a: &Vec<i32>) -> bool {
    let mut ans = false;
    for i in 0..n {
        for j in 0..n {
            if j == i {
                continue;
            }
            for k in 0..n {
                if k == j || k == i {
                    continue;
                }
                if a[i] + a[j] + a[k] == 1000 {
                    ans = true;
                }
            }
        }
    }
    ans
}