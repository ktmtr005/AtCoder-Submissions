use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        t: u64,
        mut a:[u64; n],
    }
    a.insert(0, 0);
    let ans = solve(t, a);
    println!("{} {}", ans.0, ans.1);
}
fn solve(t: u64, a: Vec<u64>) -> (usize, u64) {
    let sum: Vec<u64> = a
        .iter()
        .scan(0, |cum, n| {
            *cum += n;
            Some(*cum)
        })
        .collect();
    let playlist_all = *sum.last().unwrap();
    let cur = t % playlist_all;
    let song = sum.partition_point(|&x| cur > x);
    let sec = cur - sum[song - 1];
    (song, sec)
}