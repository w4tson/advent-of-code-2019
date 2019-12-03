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
    instructions: Vec<Instruction>,
    positions: Vec<Point>
}

#[derive(Clone,PartialEq,Debug)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn move_by(&self, i: &Instruction) -> Point {
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

    fn new(instructions: Vec<Instruction>) -> Wire {
        let positions = Wire::calc_positions(&instructions);
        Wire { instructions, positions }
    }

    fn calc_positions(instructions: &Vec<Instruction>) -> Vec<Point> {
        let mut p = Point { x:0, y: 0 };
        instructions.iter()
            .flat_map(|i| {
                let new_position = p.move_by(i);
                let points = p.points_between(&i);
                p = new_position;
                points
            }).collect()
    }
    
    pub fn first_overlap(&self, other: &Wire) -> i32 {
        self.positions.iter().skip(1)
            .filter(|p| other.positions.contains(p))
            .map(Point::dist)
            .min()
            .expect("Should have a minimum overlap")
    }

    pub fn all_intersections(&self, other: &Wire) -> Vec<Point> {
        self.positions.iter().skip(1)
            .filter(|p| other.positions.contains(p))
            .cloned()
            .collect()
    }

    pub fn steps_to(&self, position: &Point) -> usize {
        self.positions.iter().take_while(|&p| p != position).count()
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

        let distance = distance.parse().map_err(|_| ())?;

        Ok( Instruction { direction, distance })
    }
}

impl FromStr for Wire {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions  = s.split(",")
            .filter_map(|s| s.parse().ok())
            .collect();

        Ok(Wire::new(instructions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let w : Wire = "D99,L45".parse().unwrap();
        assert_eq!(w.instructions.len(), 2);
        assert_eq!(w.instructions.get(0).unwrap().direction, Down);
        assert_eq!(w.instructions.get(0).unwrap().distance, 99);
    }
    
    #[test]
    fn positions() {
        let w : Wire = "R8,U5,L5,D3".parse().unwrap();
        let p = w.positions;
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
        let w1 : Wire = "R8,U5,L5,D3".parse().unwrap();
        let w2 : Wire = "U7,R6,D4,L4".parse().unwrap();
        let overlap = w1.first_overlap(&w2);
        eprintln!("overlap = {:#?}", overlap);
    }
    
    #[test]
    fn points_between() {
        let p2 = Point{ x:10,y:0 };
        let pb = p2.points_between(&Instruction{direction: Right, distance: 5});

        eprintln!("pb = {:#?}", pb);

        assert_eq!(pb.len(), 5);
    }
    
    #[test]
    fn dist1() {
        let w1 : Wire = Wire::from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap();
        let w2 : Wire = Wire::from_str("U62,R66,U55,R34,D71,R55,D58,R83").unwrap();

        let overlap = w1.first_overlap(&w2);
        assert_eq!(overlap, 159);
        eprintln!("overlap = {:#?}", overlap);
    }

    #[test]
    fn all_intersections() {
        let w1 : Wire = Wire::from_str("R8,U5,L5,D3").unwrap();
        let w2 : Wire = Wire::from_str("U7,R6,D4,L4").unwrap();
        let intersections = w1.all_intersections(&w2);
        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections, vec![Point { x: 6, y: -5 }, Point { x: 3, y: -3 }]);
    }

    #[test]
    fn steps_to() {
        let w1 : Wire = Wire::from_str("R8,U5,L5,D3").unwrap();
        let w2 : Wire = Wire::from_str("R8,U5,L5,D3").unwrap();
        let point = Point { x: 3, y: -3 };
        assert_eq!(w1.steps_to(&point), 20);
        assert_eq!(w2.steps_to(&point), 20);
    }

    #[test]
    fn steps_to2() {
        let w1 : Wire = Wire::from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap();
        let w2 : Wire = Wire::from_str("U62,R66,U55,R34,D71,R55,D58,R83").unwrap();
        let point = Point { x: 3, y: -3 };

        let intersections = w1.all_intersections(&w2);
        let result = intersections.iter()
            .map(|i| w1.steps_to(i) + w2.steps_to(i))
            .min()
            .expect("Should be min");
        assert_eq!(result, 610);
    }
}


