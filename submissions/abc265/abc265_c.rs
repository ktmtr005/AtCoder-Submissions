use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h],
    }
    let ans = solve(h, w, g);
    match ans {
        Some((a, b)) => println!("{} {}", a + 1, b + 1),
        None => println!("-1"),
    }
}
fn solve(h: usize, w: usize, g: Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut visited = vec![vec![false; w]; h];
    let (mut i, mut j): (usize, usize) = (0, 0);
    loop {
        if visited[i][j] {
            return None;
        }
        visited[i][j] = true;
        match g[i][j] {
            'U' if i > 0 => {
                i -= 1;
            }
            'D' if i < h - 1 => {
                i += 1;
            }
            'L' if j > 0 => {
                j -= 1;
            }
            'R' if j < w - 1 => {
                j += 1;
            }
            _ => break,
        }
    }
    Some((i, j))
}