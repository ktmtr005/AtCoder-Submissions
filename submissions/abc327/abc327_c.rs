use std::collections::HashSet;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: [[i32; 9]; 9],
    }
    let ans = solve(&a);
    match ans {
        true => println!("Yes"),
        false => println!("No"),
    }
}
fn solve(a: &Vec<Vec<i32>>) -> bool {
    let mut ans = true;
    for i in 0..9usize {
        let mut num_exist = HashSet::new();
        for j in 0..9usize {
            num_exist.insert(a[i][j]);
        }
        if num_exist.len() != 9 {
            ans = false;
            break;
        }
    }
    for i in 0..9usize {
        let mut num_exist = HashSet::new();
        for j in 0..9usize {
            num_exist.insert(a[j][i]);
        }
        if num_exist.len() != 9 {
            ans = false;
            break;
        }
    }
    for i in [0usize, 3, 6] {
        for j in [0usize, 3, 6] {
            let mut num_exist = HashSet::new();
            for k in i..i+3 {
                for l in j..j+3 {
                    num_exist.insert(a[k][l]);
                }
            }
            if num_exist.len() != 9 {
                ans = false;
                break;
            }
        }
    }
    ans
}