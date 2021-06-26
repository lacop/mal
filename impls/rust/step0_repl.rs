use std::io::{self, BufRead, Write};

fn main() {
    loop {
        print!("user> ");
        io::stdout().lock().flush().unwrap();
        let mut buf = String::new();
        let len = io::stdin().lock().read_line(&mut buf).unwrap();
        if len == 0 {
            break
        }
        println!("{}", buf);
    }
}