use std::collections::HashMap;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i64
    }
    println!("{}", solve(n));
}
fn solve(n: i64) -> i64 {
    let mut memo: HashMap<i64, i64> = HashMap::new();
    f(n, &mut memo)
}
fn f(n: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if n == 1 {
        return 0;
    }
    if memo.get(&n) != None {
        return *memo.get(&n).unwrap();
    }
    let x = f(n / 2, memo) + f((n + 1) / 2, memo) + n;
    memo.entry(n).or_insert(x);
    return *memo.get(&n).unwrap();
}