use std::collections::VecDeque;
fn read() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}
fn main() {
    let n: usize = read().parse().unwrap();
    let uv = {
        let mut v = Vec::new();
        for _i in 0..n-1 {
            let ln: Vec<usize> = read().split_whitespace().map(|x| x.parse::<usize>().unwrap() - 1).collect();
            v.push((ln[0], ln[1]));
        }
        v
    };
    let mut g = vec![Vec::new(); n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let _g = &g[..];
    let ans = solve(&g);
    println!("{}", ans);
}
fn solve(g: &[Vec<usize>]) -> i32 {
    let mut dp = vec![-1; g.len()];
    dp[0] = 0;
    let mut root = Vec::new();
    for &i in &g[0] {
        root.push(i);
        dp[i] = 1;
    }
    while let Some(i) = root.pop() {
        let mut queue = VecDeque::new();
        queue.push_back(i);
        let mut cnt: i32 = 1;
        while let Some(j) = queue.pop_front() {
            for &k in &g[j] {
                if dp[k] == -1 {
                    queue.push_back(k);
                    cnt += 1;
                    dp[k] = cnt;
                }
            }
        }
    }
    let max_tree_size = *dp.iter().max().unwrap();
    g.len() as i32 - max_tree_size
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        let g = vec![
            vec![0, 1],
            vec![1, 0],
        ];
        assert_eq!(solve(&g), 1);
    }
}