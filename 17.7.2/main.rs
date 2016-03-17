use std::io;
use std::str::FromStr;

use std::collections::HashSet;

fn main() {
    {
        let set: HashSet<i32> = vec![1,3,5].into_iter().collect();
        // iter() と into_iter() との違いは&Tのコレクションを作るかTのコレクションを作るからしい
        // (http://qiita.com/uehaj/items/25caa06ce666fc175d99)
        // で、HashSet は後者からしか作れない
        for i in 0..8 {
            println!("{} is {} contained", i,
                     if set.contains(&i) { "" } else { "not" })
        }
    }
    println!("");
    {
        let mut sc = Scanner::new();
        let mut set = HashSet::<i32>::new();
        loop {
            let a: String = sc.cin();
            if a == "!" {
                set.insert(sc.cin());
            } else if a == "?" {
                let n = sc.cin();
                println!("{} is {} contained", n,
                         if set.contains(&n) { "" } else { "not" })
            } else {
                break
            }
        }
    }
}

#[allow(dead_code)]
struct Scanner { stdin: io::Stdin, buffer: Vec<String>, }
#[allow(dead_code)]
impl Scanner {
    fn new() -> Scanner { Scanner { stdin: io::stdin(), buffer: Vec::new() } }
    fn reserve(&mut self) {
        if self.buffer.len() == 0 {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split(char::is_whitespace).map(String::from) {
                if !w.is_empty() { self.buffer.push(w) }
            }
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        self.reserve();
        match self.buffer[0].parse::<T>() {
            Ok(a) => { self.buffer.remove(0); a },
            Err(_) => panic!("parse err"),
        }
    }
    fn get_char(&mut self) -> char {
        self.reserve();
        let head = self.buffer[0].chars().nth(0).unwrap();
        let tail = String::from( &self.buffer[0][1..] );
        if tail.len() > 0 {
            self.buffer[0] = tail
        } else {
            self.buffer.remove(0);
        }
        head
    }
}
