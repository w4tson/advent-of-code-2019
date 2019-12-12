use std::str::FromStr;
use itertools::Itertools;

type Position = (i32, i32, i32);
type Velocity = (i32, i32, i32);

const test_input : &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

fn main() {
}

#[derive(Debug, Clone)]
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

    pub fn kinetic_energy(&self) -> i32 {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }

    pub fn potential_energy(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }

    fn calc_gravity(a: i32, b: i32) -> i32 {
        if a < b { -1 } else if a > b { 1 } else { 0 }
    }
}


impl Space {
    pub fn tick_for(&mut self, n: usize) {
        (0..n).for_each(|_| self.tick())
    }

    pub fn tick(&mut self) {
        (0..self.moons.len()).combinations(2).for_each(|moon_tuple|{

            let mut moon1 = &mut self.moons[moon_tuple[0]];
            let moon2 =  &self.moons[moon_tuple[1]];
            moon1.apply_gravity(&moon2);
//            moon2.apply_gravity(&moon1);
        });

        &self.moons.iter_mut().for_each(|moon| moon.apply_velocity());
    }

    pub fn kinetic_energy(&self) -> i32 {
        self.moons.iter().map(|moon| moon.kinetic_energy()).sum()
    }

    pub fn potential_energy(&self) -> i32 {
        self.moons.iter().map(|moon| moon.kinetic_energy()).sum()
    }

    pub fn total_energy(&self) -> i32 {
        self.kinetic_energy() + self.potential_energy()
    }
}

impl FromStr for Space {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let moons = input
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
        .inspect(|x| eprintln!("x = {:#?}", &x))
        .collect::<Vec<i32>>();
    eprintln!("nums = {:#?}", n);
    Moon { id, position: (n[0], n[1], n[2]), velocity: (0, 0, 0) }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let space : Space = test_input.parse().unwrap();
        eprintln!("space = {:#?}", space);
    }

    #[test]
    fn total_energy() {
        let space : Space = test_input.parse().unwrap();
        eprintln!("energy = {:#?}", space.total_energy());
    }

    #[test]
    fn combos() {
        let x = vec![1,2,3,4,5];
//        x.iter().combinations(2)
    }
}