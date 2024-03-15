use proconio::{fastout, input, marker::Chars};
static NEXT_CHAR_POS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, 1),
    (-1, -1),
    (1, -1),
];
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let ans = solve(h, w, s);
    match ans {
        Some(v) => println!(
            "{}",
            v.iter()
                .map(|x| [x.0, x.1]
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" "))
                .collect::<Vec<_>>()
                .join("\n")
        ),
        None => unreachable!(),
    }
}
fn solve(h: usize, w: usize, s: Vec<Vec<char>>) -> Option<Vec<(i32, i32)>> {
    for (i, si) in s.iter().enumerate() {
        for (j, &c) in si.iter().enumerate() {
            if c == 's' {
                let snuke_pos = find_snuke(i as i32, j as i32, h as i32, w as i32, &s);
                if let Some(v) = snuke_pos {
                    return Some(v);
                }
            }
        }
    }
    None
}
fn find_snuke(
    row: i32,
    col: i32,
    row_limit: i32,
    col_limit: i32,
    s: &[Vec<char>],
) -> Option<Vec<(i32, i32)>> {
    for &(dr, dc) in &NEXT_CHAR_POS {
        let mut like_snuke = String::new();
        let mut like_snuke_pos = Vec::new();
        {
            let mut next_row = Some(row);
            let mut next_col = Some(col);
            while let (Some(r), Some(c)) = (next_row, next_col) {
                like_snuke.push(s[r as usize][c as usize]);
                like_snuke_pos.push((r + 1, c + 1));
                match r + dr {
                    i if 0 <= i && i < row_limit => next_row = Some(i),
                    _ => next_row = None,
                };
                match c + dc {
                    i if 0 <= i && i < col_limit => next_col = Some(i),
                    _ => next_col = None,
                };
                if like_snuke.len() == 5 {
                    break;
                }
            }
        }
        if like_snuke == "snuke" {
            return Some(like_snuke_pos);
        }
    }
    None
}