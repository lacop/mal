use std::io::{self, BufRead, Write};

fn read(input: String) -> String {
    input
}

fn eval(input: String) -> String {
    input
}

fn print(input: String) -> String {
    input
}

fn rep(input: String) -> String {
    print(eval(read(input)))
}

fn main() {
    loop {
        print!("user> ");
        io::stdout().lock().flush().unwrap();
        let mut buf = String::new();
        let len = io::stdin().lock().read_line(&mut buf).unwrap();
        if len == 0 {
            break
        }
        println!("{}", rep(buf));
    }
}