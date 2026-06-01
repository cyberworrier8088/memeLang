mod execute;

use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <file.dr>");
        return;
    }

    let filename = &args[1];

    let code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            println!("Failed to read file: {}", err);
            return;
        }
    };

    let mut variables: HashMap<String, String> = HashMap::new();

    let mut last_if_result = false;

    let lines: Vec<&str> = code.lines().collect();

    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        execute::execute_line(
            line,
            &mut variables,
            i + 1,
            &mut last_if_result,
        );

        i += 1;
    }
}