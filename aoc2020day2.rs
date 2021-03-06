use std::collections::VecDeque;
use std::io;

struct Scan {
    buffer: std::collections::VecDeque<String>,
}

impl Scan {
    fn new() -> Scan {
        Scan {
            buffer: VecDeque::new(),
        }
    }

    fn next_line(&self) -> io::Result<String> {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line)? {
            0 => Err(io::Error::new(io::ErrorKind::Other, "EOF")),
            _ => Ok(line),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> io::Result<T> {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                break match token.parse::<T>() {
                    Ok(x) => Ok(x),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "parsing is fucked")),
                };
            }
            let line = self.next_line()?;
            self.buffer = line.split_whitespace().map(String::from).collect();
        }
    }
}

fn _main() -> io::Result<()> {
    let mut scan = Scan::new();
    let mut result = 0usize;
    loop {
        let policy: String = match scan.next() {
            Ok(x) => x,
            _ => break,
        };
        let separator = policy.find('-').unwrap();
        let pol1: usize = (&policy[0..separator]).parse().unwrap();
        let pol2: usize = (&policy[(separator + 1)..]).parse().unwrap();
        let _letter: String = scan.next().unwrap();
        let letter: char = _letter.chars().next().unwrap();
        let pass: String = scan.next().unwrap();
        let mut count = 0usize;
        for c in pass.chars() {
            if c == letter {
                count += 1;
            }
        }
        if count >= pol1 && count <= pol2 {
            result += 1;
        }
    }
    println!("{}", result);
    Ok(())
}

fn main() -> io::Result<()> {
    std::thread::Builder::new()
        .stack_size(1 << 23)
        .spawn(_main)
        .unwrap()
        .join()
        .unwrap()?;
    Ok(())
}
