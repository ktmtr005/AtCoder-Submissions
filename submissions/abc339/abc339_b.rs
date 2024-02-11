use proconio::{input, fastout};
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }
    let ans = solve(h, w, n);
    for i in ans {
        println!("{}", i.iter().collect::<String>());
    }
}
struct Takahashi {
    row: usize,
    col: usize,
    direction: usize,
}
impl Takahashi {
    fn paint(&self, grid: &Vec<Vec<i32>>) -> i32 {
        (grid[self.row][self.col] + 1) % 2
    }
    fn rotate(&self, grid: &Vec<Vec<i32>>, current_direction: usize) -> usize {
        let color = grid[self.row][self.col];
        let mut new_direction = current_direction;
        if color == 0 {
            new_direction += 3;
        }
        else {
            new_direction += 1;
        }
        new_direction % 4
    }
    fn move_grid(&self, direction: usize, h: usize, w: usize) -> (usize, usize) {
        match direction {
            0 => ((self.row + h - 1) % h, self.col),
            1 => (self.row, (self.col + 1) % w),
            2 => ((self.row + 1) % h, self.col),
            3 => (self.row, (self.col + w - 1) % w),
            _ => panic!("invalid direction"),
        }
    }
}
fn solve(h: usize, w: usize, n: usize) -> Vec<Vec<char>> {
    let mut grid = vec![vec![0i32; w]; h];
    let mut takahashi = Takahashi {
        row: 0,
        col: 0,
        direction: 0,
    };
    for _i in 0..n {
        grid[takahashi.row][takahashi.col] = takahashi.paint(&grid);
        takahashi.direction = takahashi.rotate(&grid, takahashi.direction);
        (takahashi.row, takahashi.col) = takahashi.move_grid(takahashi.direction, h, w);
    }
    let mut ans = vec![vec!['.'; w]; h];
    for i in 0..h {
        for j in 0..w {
            ans[i][j] = convert(i, j, &grid);
        }
    }
    ans
}
fn convert(i: usize, j: usize, grid: &Vec<Vec<i32>>) -> char {
    match grid[i][j] {
        0 => '.',
        1 => '#',
        _ => panic!("invalid value"),
    }
}