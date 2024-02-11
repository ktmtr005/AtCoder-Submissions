use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        t: usize,
        n: usize,
        query: [(usize, usize); n],
    }
    let ans = solve(t, &query);
    for i in &ans {
        println!("{}", *i);
    }
}
fn solve(t: usize, query: &Vec<(usize, usize)>) -> Vec<i32> {
    let mut worker = vec![0; t + 1];
    for &(l, r) in query {
        worker[l] += 1;
        worker[r] -= 1;
    }
    let mut worker_sum = vec![0; t + 1];
    let mut tmp = 0;
    for (i, &v) in worker.iter().enumerate() {
        tmp += v;
        worker_sum[i] = tmp;
    }
    worker_sum.pop();
    worker_sum
}