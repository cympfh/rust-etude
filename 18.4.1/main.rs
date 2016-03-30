use std::path::Path;
use std::io::Read;
use std::fs::File;

fn main() {
    let path = Path::new("hello.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!("{}", why),
        // if hello.txt not exists:
        // 'No such file or directory (os error 2)'
        Ok(file) => file
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("{}", why),
        _ => println!("{}", s)
    };
}

