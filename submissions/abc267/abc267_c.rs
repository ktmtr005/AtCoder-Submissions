use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }
    let ans = solve(m, a);
    println!("{ans}");
}
fn solve(m: usize, a: Vec<i64>) -> i64 {
    let sum: Vec<i64> = a
        .iter()
        .scan(0, |cum, &x| {
            *cum += x;
            Some(*cum)
        })
        .collect();
    let sum_i = a
        .iter()
        .take(m)
        .enumerate()
        .fold(0, |acc, (i, &x)| acc + (i + 1) as i64 * x);
    let mut ans = Vec::new();
    ans.push(sum_i);
    for (i, &v) in a.iter().skip(m).enumerate() {
        let tmp = m as i64 * v + *ans.last().unwrap()
            - (sum[m + i - 1] - *sum.get(usize::wrapping_sub(i, 1)).unwrap_or(&0));
        ans.push(tmp);
    }
    *ans.iter().max().unwrap()
}