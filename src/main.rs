use std::{process, env, fs};

mod lexer;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // analyze arguments
    if args.len() == 2 {
        // use for provided source file compilation
        let source = fs::read_to_string(&args[1]).expect("Error loading source file");
    
    } else if args.len() > 2 {
        // TODO: analyze tags and commands for debugging
    } else {
        println!("Usage: owl r [file path] -tags");
        process::exit(1)
    }
}