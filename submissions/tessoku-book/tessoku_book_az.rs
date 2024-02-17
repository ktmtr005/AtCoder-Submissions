use std::collections::VecDeque;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut que = VecDeque::new();
    for _i in 0..q {
        input! {
            query_type: usize,
        }
        if query_type == 1 {
            input! {
                x: String,
            }
            que.push_back(x);
        }
        else if query_type == 2 {
            println!("{}", *que.iter().next().unwrap());
        }
        else if query_type == 3 {
            que.pop_front();
        }
    }
}