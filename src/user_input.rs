use std::io::*;

pub fn input() -> String {
    let mut input = String::new();

    print!(">> ");
    flush_output();
    stdin().read_line(&mut input).expect("Failed to read line");

    if input.ends_with('\n') {
        input.pop();
    }

    input
}

pub fn flush_output() {
    stdout().flush().unwrap();
}
