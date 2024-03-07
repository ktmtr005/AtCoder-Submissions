use std::collections::VecDeque;
use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
    }
    let ans = solve(s);
    println!("{}", ans);
}
fn solve(s: Vec<Vec<char>>) -> u32 {
    let mut ret: u32 = 0;
    let (h, w) = (s.len(), s[0].len());
    let interacted: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut visited = vec![vec![false; w]; h];
    for (row, ln) in s.iter().enumerate() {
        for (col, &v) in ln.iter().enumerate() {
            let mut q = VecDeque::new();
            if v == '.' || visited[row][col] == true {
                continue;
            }
            q.push_back((row, col));
            visited[row][col] = true;
            while let Some((row2, col2)) = q.pop_front() {
                for &(x, y) in &interacted {
                    if let Ok(n_row) = usize::try_from(row2 as i32 + y) {
                        if let Ok(n_col) = usize::try_from(col2 as i32 + x) {
                            if let Some(i) = s.get(n_row) {
                                if let Some(&c) = i.get(n_col) {
                                    if c == '#' && visited[n_row][n_col] == false {
                                        q.push_back((n_row, n_col));
                                        visited[n_row][n_col] = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            ret += 1;
        }
    }
    ret
}