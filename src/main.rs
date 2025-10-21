use std::env::args;
use std::fs;
mod interpreter;

fn main() {
    match args().count() {
        0 | 1 => panic!("requires multiple args"), // TO-DO: implement REPL
        _ => {
            let path = args().last().unwrap();
            let bytes = fs::read(path).expect("last arg should be valid path");
            interpreter::interpret(bytes);
        }
    }
}
