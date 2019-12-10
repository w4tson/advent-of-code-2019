use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};
use itertools::Itertools;
use std::collections::HashMap;

type Coord = (usize, usize);

pub struct AsteroidField {
    field: Vec<Vec<MonitoringStation>>,
    width: usize,
    height: usize
}

pub struct MonitoringStation {
    coord: Coord,
    has_asteroid: bool,
    peers: Vec<Path>
}

#[derive(Copy, Clone)]
pub struct Path {
    length: f32,
    start: Coord,
    end: Coord
}

impl Path {
    
    pub fn new(start: Coord, end: Coord) -> Path {
        let length = Self::length(start, end);
        Path { start, end, length }
    }
    
    fn degrees(&self) -> f32 {
        let opposite = self.end.1 as f32 - self.start.1 as f32;
        let adjacent = self.end.0 as f32 - self.start.0 as f32;
        let tan = (opposite/adjacent).atan().to_degrees();
        let extra = if adjacent < 0.0 { 180.0 } else { 0.0 };
        tan + 90.0 + extra
    }
    
    pub fn length(start: Coord, end: Coord ) -> f32 {
        let opposite = (end.1 as f32 - start.1 as f32).abs();
        let adjacent = (end.0 as f32 - start.0 as f32).abs();
        (opposite.powi(2) + adjacent.powi(2)).sqrt()
    } 
}


impl AsteroidField {
    
    pub fn calc_best(&mut self) -> (Coord, usize) {
        let mut bestest = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.field[y][x].has_asteroid {
                    let visible = self.visible_asteroid((x, y));
                    bestest.push((visible, (x,y)));
                }
            }
        }

        let (amount, winner) = bestest.iter()
            .max_by_key(|(a, _)| a)
            .expect("No max found");

        (*winner, *amount)
    }
    
    pub fn visible_asteroid(&mut self, coord: Coord) -> usize {
        let mut visible: Vec<Path> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.field[y][x].has_asteroid && (x,y) != coord {
                    let path = Path::new(coord,  (x,y) );
                    visible.push(path);
                }
            }
        }
        
        visible
            .iter()
            .unique_by(|item| format!("{:.2}", item.degrees()) )
            .count()
    }
}






#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn path() {
        
        let up = Path::new((1,1), (1,0)).degrees();
        let top_right = Path::new((1,1), (2,0)).degrees();
        let right = Path::new((1,1), (2,1)).degrees();
        let bottom_right = Path::new((1,1), (2,2)).degrees();
        let down = Path::new((1,1), (1,2)).degrees();
        let bottom_left = Path::new((1,1), (0,2)).degrees();
        let left = Path::new((1,1), (0,1)).degrees();
        let top_left = Path::new((1,1), (0,0)).degrees();

        assert_eq!(up, 0.0);
        assert_eq!(top_right, 45.0);
        assert_eq!(right, 90.0);
        assert_eq!(bottom_right, 135.0);
        assert_eq!(down, 180.0);
        assert_eq!(bottom_left, 225.0);
        assert_eq!(left, 270.0);
        assert_eq!(top_left, 315.0);

        eprintln!("up = {:}", Path::length((1,1),(1,0)));
        eprintln!("top right = {:}", Path::length((1,1),(2,0)));
    }

    #[test]
    fn test() {
        let input = ".#..#
.....
#####
....#
...##";
        let mut asteroid_field : AsteroidField = input.parse().unwrap();
        println!("{}", asteroid_field);
        let peers = asteroid_field.visible_asteroid((3, 4));

        assert_eq!(peers, 8);

        let (winner, amount) = asteroid_field.calc_best();
        assert_eq!(winner, (3, 4));
        assert_eq!(amount, 8);
    }
    
    #[test]
    fn example2() {
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

        let mut asteroid_field : AsteroidField = input.parse().unwrap();
        let (winner, amount) = asteroid_field.calc_best();
        assert_eq!(winner, (5, 8));
        assert_eq!(amount, 33);
    }
    
    #[test]
    fn example3() {
        let input ="#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

        let mut asteroid_field : AsteroidField = input.parse().unwrap();
        let (winner, amount) = asteroid_field.calc_best();
        assert_eq!(winner, (1,2));
        assert_eq!(amount, 35);
    }
    
    #[test]
    fn example4() {
        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

        let mut asteroid_field : AsteroidField = input.parse().unwrap();
        let (winner, amount) = asteroid_field.calc_best();
        assert_eq!(winner, (6,3));
        assert_eq!(amount, 41);
    }
    
    #[test]
    fn example5() {
        let input = std::fs::read_to_string("example5.txt").unwrap();let mut asteroid_field : AsteroidField = input.parse().unwrap();
        let (winner, amount) = asteroid_field.calc_best();
        assert_eq!(winner, (11,13));
        assert_eq!(amount, 210);
        
    }
}

impl FromStr for AsteroidField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field = s.lines().enumerate()
            .map(to_stations)
            .collect::<Vec<Vec<MonitoringStation>>>();
        
        let height = field.len();
        let width = field[0].len();
        
        Ok(AsteroidField { field, width, height})
    }
}

fn to_stations(y_line: (usize, &str)) -> Vec<MonitoringStation>{
    y_line.1.chars()
        .enumerate()
        .map(|(x, c)| to_monitoring_station((x, y_line.0), c))
        .collect()
}

fn to_monitoring_station(coord: Coord, c: char) -> MonitoringStation {
    let has_asteroid = c == '#';
    MonitoringStation { coord, has_asteroid, peers: vec![] }
}

impl Display for AsteroidField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.field[y][x].has_asteroid { "#" } else { "." };
                write!(f, "{}", c);
            }
            writeln!(f);
        }
        Ok(())
    }
}

