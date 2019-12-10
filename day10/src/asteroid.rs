use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};

type Coord = (usize, usize);

pub struct AsteroidField {
    field: Vec<Vec<MonitoringStation>>,
    width: usize,
    height: usize
}

pub struct MonitoringStation {
    has_asteroid: bool,
    peers: Vec<Path>
}

pub struct Path {
    start: Coord,
    end: Coord
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
    MonitoringStation { has_asteroid, peers: vec![] }
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let input = ".#..#
.....
#####
....#
...##";
        let asteroid_field : AsteroidField = input.parse().unwrap();
        println!("{}", asteroid_field);
        
        
    }
}