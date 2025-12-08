use std::env::args;
use std::fs;
mod interpreter;

fn main() {
    match args().count() {
        0 | 1 => panic!("requires file path arguement"), // TO-DO: implement REPL
        2 => {
            let path = args().last().unwrap();
            let bytes = fs::read(path).expect("last arg should be valid file path");
            interpreter::interpret(bytes);
        }
        _ => panic!("too many arguements"),
    }
}
