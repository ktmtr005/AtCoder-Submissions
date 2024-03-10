use std::collections::HashMap;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let ans = solve(a);
    println!("{}", ans.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "));
}
fn solve(a: Vec<u32>) -> Vec<u32> {
    let mut front = HashMap::new();
    let mut back = HashMap::new();
    for i in 1..a.len() {
        front.insert(a[i], a[i-1]);
    }
    for i in 0..a.len()-1 {
        back.insert(a[i], a[i+1]);
    }
    input! {
        q: usize,
    }
    let mut head = a[0];
    for _i in 0..q {
        input! {
            query_type: usize,
        }
        match query_type {
            1 => {
                input! {
                    x: u32,
                    y: u32,
                }
                if let Some(&i) = back.get(&x) {
                    back.insert(y, i);
                    front.insert(i, y);
                }
                back.insert(x, y);
                front.insert(y, x);
            },
            2 => {
                input! {
                    x: u32,
                }
                let x_front = front.get(&x);
                let x_back = back.get(&x);
                match (x_front, x_back) {
                    (Some(&i), Some(&j)) => {
                        front.insert(j, i);
                        back.insert(i, j);
                        front.remove(&x);
                        back.remove(&x);
                    },
                    (Some(&i), None) => {
                        front.remove(&x);
                        back.remove(&i);
                    },
                    (None, Some(&j)) => {
                        back.remove(&x);
                        front.remove(&j);
                        head = j;
                    },
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }
    let mut ret = Vec::new();
    ret.push(head);
    while let Some(&i) = back.get(&head) {
        ret.push(i);
        head = i;
    }
    ret
}