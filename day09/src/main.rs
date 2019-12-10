use std::fs;
use crate::opcode::Program;

pub mod opcode;

fn main() {
    let input : Vec<i64> = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    
    let mut p = Program::new(&input);
    p.exec();
}
    