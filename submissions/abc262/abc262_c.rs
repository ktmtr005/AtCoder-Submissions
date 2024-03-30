use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let ans = solve(a);
    println!("{ans}");
}
fn solve(a: Vec<usize>) -> usize {
    let same = a.iter().enumerate().filter(|&(i, &v)| i == v).count();
    let mut cnt = (same * (same - 1)) / 2;
    for (i, &j) in a.iter().enumerate().filter(|&(x, &y)| x != y) {
        if (a[i] == j) && (a[j] == i) && (a[i] > i) {
            cnt += 1;
        }
    }
    cnt
}