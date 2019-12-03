use std::fs;
use std::str::FromStr;
use nom::sequence::{tuple, pair};
use nom::multi::separated_list;
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::character::complete::{alpha1, digit1};
use nom::bytes::complete::tag;
use crate::wire::Direction::{Down, Up, Right, Left};
use std::cmp::{min, max};
use num::range_step;

#[derive(Debug)]
pub struct Wire {
    instructions: Vec<Instruction>
}

#[derive(Clone,PartialEq,Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn move_p(&self, i: &Instruction) -> Point {
        match i.direction {
            Up    => Point{ x: self.x , y: self.y - i.distance},
            Down  => Point{ x: self.x , y: self.y + i.distance},
            Left  => Point{ x: self.x - i.distance, y: self.y},
            Right => Point{ x: self.x + i.distance, y: self.y}
        }
    }
    
    fn points_between(&self, i: &Instruction) -> Vec<Point> {
        match i.direction {
            Up    => {
                range_step(self.y, self.y-i.distance, -1).map(|y| Point{ x: self.x, y }).collect()
            },
            Down  => {
                (self.y..self.y+i.distance).map(|y| Point{ x: self.x, y }).collect()
            },
            Left  => {
                range_step(self.x,self.x - i.distance, -1).map(|x| Point{ x, y: self.y }).collect()
            },
            Right => {
                (self.x..self.x+i.distance).map(|x| Point{ x, y: self.y }).collect()
            }
        }
    }
    
    fn dist(&self) -> i32 {
        self.y.abs() + self.x.abs()
    }
}

impl Wire {
    fn positions(&self) -> Vec<Point> {
        let mut p = Point { x:0, y: 0};
        self.instructions.iter()
            .flat_map(|i| {
                let new_p = p.move_p(i);
//                eprintln!("new_p = {:#?}", new_p);
                let points = p.points_between(&i);
                p = new_p;
                points
            }).collect()
    }
    
    pub fn first_overlap(&self, other: &Wire) -> i32 {
        let positions1 = self.positions();
        let positions2 = other.positions();
//        eprintln!("positions1 = {:#?}", positions1);
//        eprintln!("positions1 = {:#?}", positions2);
        
        let dist = positions1.iter().skip(1)
            .filter(|p| positions2.contains(p))
            .map(|p| p.dist())
            .min()
            .expect("Should have a minimum overlap");
//        eprintln!("p = {:#?}", p);
//        let x = p.get(0).unwrap();
        dist
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i32
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_at(1);
        let direction = match direction {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err(())
        }?;

        let distance = distance.parse::<i32>().unwrap();

        Ok( Instruction { direction, distance })
    }
}

impl FromStr for Wire {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions : Vec<&str> = s.split(",").collect();
        let instructions : Result<Vec<Instruction>, ()> = instructions.iter().map(|s| Instruction::from_str(s)).collect();
        let instructions = instructions?;

        Ok(Wire{ instructions })
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let w : Wire = Wire::from_str("D99,L45").unwrap();
        assert_eq!(w.instructions.len(), 2);
        assert_eq!(w.instructions.get(0).unwrap().direction, Down);
        assert_eq!(w.instructions.get(0).unwrap().distance, 99);
    }
    
    #[test]
    fn positions() {
        let input = "R8,U5,L5,D3";
        let w : Wire = Wire::from_str("R8,U5,L5,D3").unwrap();
        let p = w.positions();
        assert_eq!(&p[..14], [
            Point{ x: 0, y: 0},
            Point{ x: 1, y: 0},
            Point{ x: 2, y: 0},
            Point{ x: 3, y: 0},
            Point{ x: 4, y: 0},
            Point{ x: 5, y: 0},
            Point{ x: 6, y: 0},
            Point{ x: 7, y: 0},
            Point{ x: 8, y: 0},
            Point{ x: 8, y: -1},
            Point{ x: 8, y: -2},
            Point{ x: 8, y: -3},
            Point{ x: 8, y: -4},
            Point{ x: 8, y: -5},
        ]);
    }
    
    #[test]
    fn overlap() {
        let w1 : Wire = Wire::from_str("R8,U5,L5,D3").unwrap();
        let w2 : Wire = Wire::from_str("U7,R6,D4,L4").unwrap();
        let overlap = w1.first_overlap(&w2);
        eprintln!("overlap = {:#?}", overlap);
    }
    
    #[test]
    fn points_between() {
        let p2 = Point{ x:10,y:0 };
        let pb = p2.points_between(&Instruction{direction: Right, distance: 5});

        eprintln!("pb = {:#?}", pb);

        assert_eq!(pb.len(), 4);
    }
    
    #[test]
    fn dist1() {
        let w1 : Wire = Wire::from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap();
        let w2 : Wire = Wire::from_str("U62,R66,U55,R34,D71,R55,D58,R83").unwrap();

        let overlap = w1.first_overlap(&w2);
        assert_eq!(overlap, 159);
        eprintln!("overlap = {:#?}", overlap);
    }
}




//        let result: IResult<&str, Vec<Instruction>> = separated_list(
//            tag("tag"),
//            map_res(pair(alpha1,
//                 map_res(digit1,
//                         |s: &str| s.parse::<i32>())),
//            |(direction,distance)| Ok(Instruction { direction: Direction::Up, distance: distance }))
//        )(s);