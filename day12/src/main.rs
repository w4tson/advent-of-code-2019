use std::str::FromStr;
use itertools::Itertools;
use std::fmt::{Display, Formatter, Error};

type Position = (i32, i32, i32);
type Velocity = (i32, i32, i32);

const TEST_INPUT: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

const PUZZLE_INPUT: &str =  "<x=12, y=0, z=-15>
<x=-8, y=-5, z=-10>
<x=7, y=-17, z=1>
<x=2, y=-11, z=-6>";

fn main() {
    let mut space : Space = PUZZLE_INPUT.parse().unwrap();
    space.tick_for(1000);
    eprintln!("space.total_energy() = {:#?}", space.total_energy());
}

#[derive(Debug, Clone, PartialEq)]
struct Moon {
    id: usize,
    position: Position,
    velocity: Velocity
}

#[derive(Debug)]
struct Space {
    moons: Vec<Moon>
}

impl Moon {
    pub fn apply_gravity(&mut self, other: &Moon) {
        let x = Moon::calc_gravity(self.position.0, other.position.0);
        let y = Moon::calc_gravity(self.position.1, other.position.1);
        let z = Moon::calc_gravity(self.position.2, other.position.2);
        self.velocity.0 += x;
        self.velocity.1 += y;
        self.velocity.2 += z;
    }

    pub fn apply_velocity(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    pub fn  kinetic_energy(&self) -> i32 {
        let k = self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs();
        eprintln!("k = {:#?}", k);
        k
    }

    pub fn potential_energy(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }
    
    pub fn energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn calc_gravity(a: i32, b: i32) -> i32 {
        if a < b { 1 } else if a > b { -1 } else { 0 }
    }
}


impl Space {
    pub fn tick_for(&mut self, n: usize) {
//        eprintln!("After 0" );
//        self.moons.iter().for_each(|moon| eprintln!("{}", moon));
        (1..=n).for_each(|step| {
        
            self.tick();
//            eprintln!("After {}", step );
//            self.moons.iter().for_each(|moon| eprintln!("{}", moon));
        })
    }

    fn tick(&mut self) {
        let initial : Vec<Moon> = self.moons.clone();
        let other_moons = self.moons.clone();
        (0..self.moons.len()).combinations(2).for_each(| mut moon_tuple|{

            let moon1 = &mut self.moons[moon_tuple[0]];
            let moon2 = other_moons.iter().find(|moon| moon.id == moon_tuple[1]).unwrap();
                
            moon1.apply_gravity(&moon2);

            let moon1 = other_moons.iter().find(|moon| moon.id == moon_tuple[0]).unwrap();
            let moon2 = &mut self.moons[moon_tuple[1]];

            moon2.apply_gravity(&moon1);
        });
        
        

        &self.moons.iter_mut().for_each(|moon| moon.apply_velocity());

        
    }
    
    fn print_positions(&self) {
        self.moons.iter().for_each(|m1|{
            eprintln!("{},{},{}",
                      m1.position.0,
                      m1.position.1,
                      m1.position.2 
            );
        });
    }
    
    fn print_change_of_position(&self) {
        self.moons.iter().zip(initial.iter()).take(1).for_each(|(m1, m2)|{
            eprintln!("{},{},{}",
                      m1.position.0 - m2.position.0,
                      m1.position.1 - m2.position.1,
                      m1.position.2 - m2.position.2
            );
        });
    }
    
    
    pub fn total_energy(&self) -> i32 {
        self.moons.iter().map(|moon| moon.energy()).sum()
    }
}

impl FromStr for Space {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let moons = PUZZLE_INPUT
            .lines()
            .enumerate()
            .map(|(i, s)| parse(i, s))
            .collect::<Vec<Moon>>();
        Ok(Space{ moons })
    }
}


fn parse(id: usize, line: &str) -> Moon {
    let mut line = line;
    let line = line.replace("<","")
        .replace(">","");
    let n = line.split(",")
        .map(|s| s.trim())
        .map(|s| {
            let s = &s[2..].to_string();
            s.parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>();
    Moon { id, position: (n[0], n[1], n[2]), velocity: (0, 0, 0) }
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for moon in &self.moons {
            writeln!(f, "{}", moon);
        }
        
        Ok(())
    }
}

impl Display for Moon {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "pos=<x={:2}, y={:2}, z={:2}>, vel=<x={:2}, y={:2}, z={:2}>", 
                 self.position.0,
                 self.position.1,
                 self.position.2,
                 self.velocity.0,
                 self.velocity.1,
                 self.velocity.2,
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let space : Space = TEST_INPUT.parse().unwrap();
        eprintln!("space = {:#?}", space);
    }

    #[test]
    fn total_energy() {
        let mut space : Space = TEST_INPUT.parse().unwrap();
        space.tick_for(10);
        assert_eq!(space.total_energy(), 179);
    }

    #[test]
    fn test2() {
        let input2 = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

        let mut space : Space = input2.parse().unwrap();
        space.tick_for(100);
        assert_eq!(space.total_energy(), 1940);
    }

    #[test]
    fn part2() {
        let mut space : Space = TEST_INPUT.parse().unwrap();
        space.tick_for(2772);
    }
}