use std::fs;
use crate::wire::Wire;
use std::str::FromStr;

mod wire;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let lines : Vec<&str> = input
        .lines()
        .collect();

    let w1 : Wire = Wire::from_str(lines.get(0).unwrap()).unwrap();
    let w2 : Wire = Wire::from_str(lines.get(1).unwrap()).unwrap();

    let overlap = w1.first_overlap(&w2);
    eprintln!("overlap = {:#?}", overlap);


}
