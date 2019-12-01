use std::env;
use std::fs;

fn main() {
    let result = part01();
    eprintln!("result = {:#?}", result);
}

fn part01() -> i32{
    // --snip--
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents.lines()
        .map(|a| a.parse::<i32>().expect("not i32"))
        .map(|mass| fuel_for_module(mass))
        .sum()
}

fn fuel_for_module(mass: i32) -> i32 {
    let x = (mass / 3) as f32;
    (x.floor() - 2.0) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test1() {
        assert_eq!(fuel_for_module(12), 2);
        assert_eq!(fuel_for_module(14), 2);
        assert_eq!(fuel_for_module(1969), 654);
        assert_eq!(fuel_for_module(100756), 33583);
    }
}