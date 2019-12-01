use std::fs;
use std::ops::{Div, Sub};

fn main() {
    let part1 = calculate_fuel(fuel_amount_01);
    let part2 = calculate_fuel(fuel_amount_02);
    eprintln!("part1 = {:#?}", part1);
    eprintln!("part2 = {:#?}", part2);
}

fn calculate_fuel<X>(solver : X) -> i32 
    where X : Fn(i32) -> i32 {

    fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .map(|a| a.parse::<i32>().expect("not i32"))
        .map(solver)
        .sum()
}

fn fuel_amount_01(mass: i32) -> i32 {
    mass.div(3).sub(2)
}

fn fuel_amount_02(mass: i32) -> i32 {
    let fuel = fuel_amount_01(mass);
    if fuel <= 0 { 0 } else { fuel + fuel_amount_02(fuel)}
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test1() {
        assert_eq!(fuel_amount_01(12), 2);
        assert_eq!(fuel_amount_01(14), 2);
        assert_eq!(fuel_amount_01(1969), 654);
        assert_eq!(fuel_amount_01(100756), 33583);
    }
    
    #[test]
    fn test2() {
        assert_eq!(fuel_amount_02(100756), 50346);
    }
}