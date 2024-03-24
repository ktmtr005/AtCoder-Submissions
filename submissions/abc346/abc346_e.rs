use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        h: i64,
        w: i64,
        m: usize,
        tax: [(i32, Usize1, usize); m],
    }
    let ans = solve(h, w, tax);
    println!(
        "{}\n{}",
        ans.len(),
        ans.iter()
            .map(|t| [t.0.to_string(), t.1.to_string()].join(" "))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
fn solve(h: i64, w: i64, tax: Vec<(i32, usize, usize)>) -> Vec<(usize, i64)> {
    let mut painted_cnt = [0_i64; 200009]; //[色個数; 色番号]
    let mut enable_paint_row = w;
    let mut enable_paint_col = h;
    let mut is_painted_row = vec![false; h as usize];
    let mut is_painted_col = vec![false; w as usize];
    let mut not_painted = h * w;
    for (t, a, x) in tax.into_iter().rev() {
        match t {
            1 => {
                if !is_painted_row[a] {
                    is_painted_row[a] = true;
                    painted_cnt[x] += enable_paint_row;
                    not_painted -= enable_paint_row;
                    enable_paint_col -= 1;
                }
            }
            2 => {
                if !is_painted_col[a] {
                    is_painted_col[a] = true;
                    painted_cnt[x] += enable_paint_col;
                    not_painted -= enable_paint_col;
                    enable_paint_row -= 1;
                }
            }
            _ => unreachable!(),
        }
    }
    painted_cnt[0] += not_painted;
    let mut ret = Vec::with_capacity(200000);
    for (i, &n) in painted_cnt.iter().enumerate() {
        if n > 0 {
            ret.push((i, n));
        }
    }
    ret
}