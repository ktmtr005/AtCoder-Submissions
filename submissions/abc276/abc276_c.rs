use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        p: [u32; n],
    }
    let ans = solve(p);
    println!(
        "{}",
        ans.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    );
}
fn solve(mut p: Vec<u32>) -> Vec<u32> {
    let mut chunk_size = 1;
    for n in p.windows(2).rev() {
        chunk_size += 1;
        if n[0] > n[1] {
            break;
        }
    }
    let chunk = p.rchunks_mut(chunk_size).next().unwrap();
    let max = *chunk.iter().filter(|&&x| x < chunk[0]).max().unwrap();
    let max_pos = chunk.iter().position(|&x| x == max).unwrap();
    chunk.swap(0, max_pos);
    chunk[1..].sort_by(|a, b| b.cmp(a));
    p
}