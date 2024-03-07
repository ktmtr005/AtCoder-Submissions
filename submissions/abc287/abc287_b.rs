use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }
    let ans = solve(s, t);
    println!("{}", ans);
}
fn solve(s: Vec<String>, t: Vec<String>) -> usize {
    let mut cnt: usize = 0;
    for i in s {
        for j in &t {
            if &i[3..6] == j {
                cnt += 1;
                break;
            }
        }
    }
    cnt
}