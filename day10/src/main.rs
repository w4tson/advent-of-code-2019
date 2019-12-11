use crate::asteroid::AsteroidField;

pub mod asteroid;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut asteroid_field : AsteroidField = input.parse().unwrap();
    let (winner, amount) = asteroid_field.calc_best();
    eprintln!("winner = {:#?} with {}", winner, amount);
}
