use proconio::input;
fn solve(n: i32, a: &mut Vec<i32>) -> i32 {
    a.sort();
    a.reverse();
    let mut alice_score = 0;
    let mut bob_score = 0;
    for i in 0..n {
        if i % 2 == 0 {
            alice_score += a[i as usize];
        }
        else {
            bob_score += a[i as usize];
        }
    }
    let score_diff = alice_score - bob_score;
    score_diff
}
fn main() {
    input! {
        n: i32,
        mut a: [i32; n],
    }
    let ans = solve(n, &mut a);
    println!("{}", ans);
}