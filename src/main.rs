use std::{process, env, fs};
use lexer::Lexer;

mod lexer;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Analyze arguments
    if args.len() == 2 {
        let src = fs::read_to_string(&args[1]).expect("Error loading source file");
        
        let mut lexer = Lexer::new(&src);
        let tokens = lexer.scan();
        dbg!(&tokens);
    
    } else if args.len() > 2 {
        // TODO: Analyze tags and commands for debugging
    } else {
        println!("Usage: ...");
        process::exit(1)
    }
}