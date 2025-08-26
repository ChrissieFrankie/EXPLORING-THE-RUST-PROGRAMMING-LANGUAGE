use std::io::{self, Read};                                                  // standard input/output library

fn main() {
    let mut code = String::new();                                           // create mutable string
    io::stdin().read_to_string(&mut code).expect("Failed to read input");   // read input from stdin until EOF

    let escaped = code.replace("\\", "\\\\").replace("\"", "\\\"");         // safely include comments

    println!("\n\nprintln!(\"{}\",);", escaped);                            // print println! that will print the original statement(s)
}
