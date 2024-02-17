use std::{cmp::Reverse, collections::BinaryHeap};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut que = BinaryHeap::new();
    for _i in 0..q {
        input! {
            query_type: usize,
        }
        if query_type == 1 {
            input! {
                x: usize,
            }
            que.push(Reverse(x));
        }
        else if query_type == 2 {
            let &value = {
                match que.peek().unwrap() {
                    Reverse(i) => i
                }
            };
            println!("{}", value);
        }
        else if query_type == 3 {
            que.pop();
        }
    }
}