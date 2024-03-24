use std::collections::VecDeque;
use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u32; n],
    }
    let ans = solve(k, a);
    println!(
        "{}",
        ans.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    );
}
fn solve(k: usize, a: Vec<u32>) -> VecDeque<u32> {
    let mut deque = VecDeque::from(a);
    for _i in 0..k {
        deque.pop_front();
        deque.push_back(0);
    }
    deque
}