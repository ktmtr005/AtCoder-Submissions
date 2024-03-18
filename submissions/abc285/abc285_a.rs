use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let ans = solve(a, b);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(a: usize, b: usize) -> bool {
    const G: [[usize; 2]; 14] = [
        [1, 2],
        [1, 3],
        [2, 4],
        [2, 5],
        [3, 6],
        [3, 7],
        [4, 8],
        [4, 9],
        [5, 10],
        [5, 11],
        [6, 12],
        [6, 13],
        [7, 14],
        [7, 15],
    ];
    let arr = [a.min(b), a.max(b)];
    return match G.iter().find(|&&x| x == arr) {
        Some(_i) => true,
        None => false,
    };
}