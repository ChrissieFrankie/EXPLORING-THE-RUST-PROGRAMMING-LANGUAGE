use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut code = String::new();

    println!("Paste your code, then type a single line with only END and press Enter:");

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if line.trim() == "END" {
            break;
        }
        code.push_str(&line);
        code.push('\n');
    }

    let escaped = code
        .replace('\\', "\\\\")
        .replace('\"', "\\\"")
        .replace('{', "{{")
        .replace('}', "}}")
        .replace('\t', "\\t")
        .replace('\n', "\\n");

    println!("println!(\"{}\")", escaped);
}
