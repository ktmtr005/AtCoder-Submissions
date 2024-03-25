fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock(), 6);
    let h: u32 = sc.next();
    let m: u32 = sc.next();
    let ans = solve(h, m);
    println!("{} {}", ans.0, ans.1);
}
fn solve(mut h: u32, mut m: u32) -> (u32, u32) {
    loop {
        let h_swap = m / 10 + h / 10 * 10;
        let m_swap = m % 10 + h % 10 * 10;
        if (0..=23).contains(&h_swap) && (0..=59).contains(&m_swap) {
            return (h, m);
        } else {
            m += 1;
            if m >= 60 {
                m = 0;
                h = (h + 1) % 24;
            }
        }
    }
}
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