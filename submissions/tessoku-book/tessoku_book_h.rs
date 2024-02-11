use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        h: i32,
        w: i32,
        x: [[i32; w]; h],
        q: i32,
        query: [(usize, usize, usize, usize); q],
    }
    let ans = solve(&x, &query);
    for i in ans {
        println!("{}", i);
    }
}
fn solve(x: &Vec<Vec<i32>>, query: &Vec<(usize, usize, usize, usize)>) -> Vec<i32> {
    let mut grid_sum = vec![vec![0; x[0].len() + 1]; x.len() + 1];
    for row in 0..x.len() {