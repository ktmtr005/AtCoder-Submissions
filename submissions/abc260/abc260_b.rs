use itertools::Itertools;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        a: [u32; n],
        b: [u32; n],
    }
    let ans = solve(x, y, z, a, b);
    println!("{}", ans.iter().join("\n"));
}
fn solve(x: usize, y: usize, z: usize, a: Vec<u32>, b: Vec<u32>) -> Vec<usize> {
    let mut score: Vec<(usize, (u32, u32))> =
        a.into_iter().zip(b.into_iter()).enumerate().collect(); // (examinne, (math, English))
    score.sort_by(|a, b| (a.1 .0, b.0).cmp(&(b.1 .0, a.0)));
    let mut admitted = Vec::new();
    for _i in 0..x {
        match score.pop() {
            Some((i, _)) => admitted.push(i + 1),
            None => break,
        }
    }
    score.sort_by(|a, b| (a.1 .1, b.0).cmp(&(b.1 .1, a.0)));
    for _i in 0..y {
        match score.pop() {
            Some((i, _)) => admitted.push(i + 1),
            None => break,
        }
    }
    score.sort_by(|a, b| (a.1 .0 + a.1 .1, b.0).cmp(&(b.1 .0 + b.1 .1, a.0)));
    for _i in 0..z {
        match score.pop() {
            Some((i, _)) => admitted.push(i + 1),
            None => break,
        }
    }
    admitted.sort();
    admitted
}