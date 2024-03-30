use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        n: usize,
        table: [Chars; n],
    }
    let ans = solve(table);
    println!("{}", if ans { "correct" } else { "incorrect" });
}
fn solve(table: Vec<Vec<char>>) -> bool {
    for (i, ln) in table.iter().enumerate() {
        for (j, &c) in ln.iter().enumerate().skip(i + 1) {
            match c {
                'W' if table[j][i] != 'L' => return false,
                'L' if table[j][i] != 'W' => return false,
                'D' if table[j][i] != 'D' => return false,
                _ => continue,
            }
        }
    }
    true
}