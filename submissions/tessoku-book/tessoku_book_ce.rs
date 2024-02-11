use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
        a: [i32; n],
        q: i32,
        query: [(usize, usize); q],
    }
    let ans = solve(&a, &query);
    for i in ans {
        if i == 0 {
            println!("win");
        }
        else if i == 1 {
            println!("lose");
        }
        else {
            println!("draw");
        }
    }
}
fn solve(a: &Vec<i32>, query: &Vec<(usize, usize)>) -> Vec<i32> {
    let mut sum_win = vec![0; a.len() + 1];
    let mut sum_lose = vec![0; a.len() + 1];
    for i in 0..a.len() {
        if a[i] == 1 {
            sum_win[i + 1] = sum_win[i] + 1; 
            sum_lose[i + 1] = sum_lose[i];
        }
        else {
            sum_lose[i + 1] = sum_lose[i] + 1;
            sum_win[i + 1] = sum_win[i];
        }
    }
    let mut ans = Vec::new(); // 0: win, 1: lose, 2: draw
    for &(l, r) in query {
        let win = sum_win[r] - sum_win[l - 1];
        let lose = sum_lose[r] - sum_lose[l - 1];
        if win > lose {
            ans.push(0);
        }
        else if win < lose {
            ans.push(1);
        }
        else {
            ans.push(2);
        }
    }
    ans
}