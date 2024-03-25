use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let ans = solve(h, w, c);
    println!(
        "{}",
        ans.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    );
}
fn solve(h: usize, w: usize, c: Vec<Vec<char>>) -> Vec<usize> {
    let c_t = (0..w)
        .map(|col| (0..h).map(|row| c[row][col]).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    c_t.iter()
        .map(|v| v.iter().filter(|&&c| c == '#').count())
        .collect()
}