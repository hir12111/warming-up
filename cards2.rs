struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: std::collections::VecDeque::new(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                break token.parse::<T>().ok().unwrap();
            }
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).expect("Fail to read");
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }
}

fn _main() {
    let mut scan = Scan::new();
    let _n: usize = scan.next();
    let input: String = scan.next();
    let mut zeros = 0usize;
    let mut ones = 0usize;
    for c in input.chars() {
        if c == 'z' {
            zeros += 1;
        } else if c == 'n' {
            ones += 1;
        }
    }
    for _ in 0..ones {
        print!("1 ");
    }
    for _ in 0..zeros {
        print!("0 ");
    }
    println!();
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap();
}
