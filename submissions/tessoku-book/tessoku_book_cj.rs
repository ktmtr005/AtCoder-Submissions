use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
        mut a: [i32; n],
        q: i32,
        x: [i32; q],
    }
    let ans = solve(&mut a, &x);
    for i in ans {
        println!("{}", i);
    }
}
fn solve(a: &mut Vec<i32>, x: &Vec<i32>) -> Vec<usize> {
    a.push(0);
    a.sort();
    let mut ans = vec![0; x.len()];
    for (i, &v) in x.iter().enumerate() {
        ans[i] = lower_bound(a, v);
    }
    ans
}
fn lower_bound(a: &Vec<i32>, v: i32) -> usize {
    let mut left = 1_usize;
    let mut right = a.len() - 1;
    let ans;
    while left < right {
        let center = (left + right) / 2;
        if a[center] < v {
            left = center + 1;
        }
        if v < a[center] {
            right = center - 1;
        }
        if a[center] == v {
            left = center;
            break;
        }
    }
    if a[left] < v {
        ans = left;
    }
    else {
        ans = left - 1;
    }
    ans
}