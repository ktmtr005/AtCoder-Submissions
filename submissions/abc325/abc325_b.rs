use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        wx: [(usize, usize); n],
    }
    let ans = solve(&wx);
    println!("{}", ans);
}
fn solve(wx: &Vec<(usize, usize)>) -> usize {
    let mut time_table = [0usize; 24];
    for &(w, x) in wx {
        let mut global_time = (24 - x) % 24;
        let global_time_end = (24 - x + 9) % 24;
        while global_time != global_time_end {
            time_table[global_time] += w;
            global_time += 1;
            global_time = global_time % 24;
        }
    }
    *time_table.iter().max().unwrap()
}