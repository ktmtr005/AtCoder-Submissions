use proconio::{input, fastout};
#[fastout]
fn main() {
    input!{
        n: i32,
    }
    let grid_size = n as usize;
    let dx: [i32; 4] = [0, 1, 0 , -1];
    let dy: [i32; 4] = [1, 0, -1, 0];
    let mut direction: usize = 1; // 0: up, 1: right, 2: down, 3: left
    let mut x = 0;
    let mut y = 0;
    let mut cur = 2;
    let mut grid = vec![vec![String::from(""); grid_size]; grid_size];
    grid[0][0] = String::from("1");
    while cur < n*n {
        let xx = x + dx[direction];
        let yy =  y + dy[direction];
        if 0 <= xx && xx < n && 0 <= yy && yy < n && grid[xx as usize][yy as usize] == "" {
            grid[xx as usize][yy as usize] = cur.to_string();
            cur += 1;
            x = xx;
            y = yy;
        }
        else {
            direction = (direction + 1) % 4;
        }
    }
    grid[grid_size / 2][grid_size / 2] = String::from("T");
    for ln in grid {
        println!("{}", ln.join(" "));
    }
}