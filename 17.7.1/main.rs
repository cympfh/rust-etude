use std::io;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Secret {
    username: String,
    password: String,
}

#[derive(Debug)]
struct User {
    nickname: String
}

fn main() {
    let mut sc = Scanner::new();
    let mut book = HashMap::<Secret, User>::new();

    loop {
        let action: String = sc.cin();
        if action == "register" {

            let username: String = sc.cin();
            let password: String = sc.cin();
            let nickname: String = sc.cin();
            // TODO: 二重登録チェック
            let s = Secret { username: username, password: password };
            let u = User { nickname: nickname };
            book.insert(s, u);

        } else if action == "login" {

            let username: String = sc.cin();
            let password: String = sc.cin();
            let s = Secret { username: username, password: password.clone() };

            match book.get(&s) {
                Some(u) => {
                    println!("succeed to login; you are {}", u.nickname)
                },
                None => {
                    println!("failed to login (w password: {})", password)
                }
            }

        } else {
            break
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
