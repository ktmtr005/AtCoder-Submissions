use itertools::Itertools;
fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock(), 25000);
    let n: usize = sc.next();
    let m: usize = sc.next();
    let x: Vec<Vec<usize>> = (0..m)
        .map(|_x| {
            let k: usize = sc.next();
            (0..k)
                .map(|_x| sc.next::<usize>() - 1)
                .collect::<Vec<usize>>() // 0-indexed
        })
        .collect::<Vec<Vec<usize>>>();
    let ans = solve(n, x);
    println!("{}", if ans { "Yes" } else { "No" });
}
fn solve(n: usize, x: Vec<Vec<usize>>) -> bool {
    let mut attended = vec![vec![false; n]; n];
    (0..n).for_each(|x| attended[x][x] = true);
    for v in &x {
        for (&a, &b) in v.iter().tuple_combinations() {
            attended[a][b] = true;
            attended[b][a] = true;
        }
    }
    attended.iter().all(|v| v.iter().all(|&x| x))
}
// snippet
struct Scanner<R: std::io::BufRead> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
}
impl<R: std::io::BufRead> Scanner<R> {
    fn new(reader: R, capacity: usize) -> Self {
        Scanner {
            reader,
            buf: Vec::with_capacity(capacity),
            pos: 0,
        }
    }
    fn next<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        if self.buf.is_empty() {
            self.read_next_line();
        }
        let mut start = None;
        loop {
            if self.pos == self.buf.len() {
                break;
            }
            match (self.buf[self.pos], start.is_some()) {
                (b' ', true) | (b'\n', true) => break,
                (_, true) | (b' ', false) => self.pos += 1,
                (b'\n', false) => self.read_next_line(),
                (_, false) => start = Some(self.pos),
            }
        }
        let elem = unsafe { std::str::from_utf8_unchecked(&self.buf[start.unwrap()..self.pos]) };
        elem.parse()
            .unwrap_or_else(|_| panic!("{}", format!("failed parsing: {}", elem)))
    }
    fn read_next_line(&mut self) {
        self.pos = 0;
        self.buf.clear();
        if self.reader.read_until(b'\n', &mut self.buf).unwrap() == 0 {
            panic!("Reached EOF");
        }
    }
}