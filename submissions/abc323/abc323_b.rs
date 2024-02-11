use proconio::{fastout, input};
use proconio::marker::Chars;
#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let ans = solve(&s);
    println!("{}", ans.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "));
}
fn solve(s: &Vec<Vec<char>>) -> Vec<usize> {
    let mut win = vec![(0, 0); s.len()]; // (勝利数, プレイヤー番号)
    for i in 0..s.len() {
        win[i] = (count_win(&s[i]), i + 1);
    }
    win.sort_by(|a, b| {
        if a.0 != b.0 {
            b.0.partial_cmp(&(a.0)).unwrap()
        }
        else {
            a.1.partial_cmp(&(b.1)).unwrap()
        }
    });
    let mut ans = vec![0usize; s.len()];
    for i in 0..s.len() {
        ans[i] = win[i].1;
    }
    ans
}
fn count_win(i: &Vec<char>) -> i32 {
    let mut cnt = 0;
    for &c in i {
        if c == 'o' {
            cnt += 1;
        }
    }
    cnt
}