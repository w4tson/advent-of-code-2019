#![feature(entry_insert)]
use std::fs;
use crate::opcode::Program;
use std::collections::HashMap;
use crate::Direction::{North, South, West, East};
use itertools::Itertools;



type Coord = (usize, usize);

pub mod opcode;

enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    pub fn rotateClockwise(&self) -> Direction {
        match self {
            North => East,
            East=> South,
            South => West,
            West => North
        }
    }

    pub fn rotateAntiClockwise(&self) -> Direction {
        match self {
            North => West,
            East=> North,
            South => East,
            West => South
        }
    }
}

fn main() {
    let op_codes: Vec<i64> = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut canvas: HashMap<Coord, i64> = HashMap::new();
    canvas.insert((0,0), 0);
    let mut position = (0,0);
    let mut p = Program::new(&op_codes);

    p.input_supplier(move || {
        canvas[&position]
    });

    let mut count = 0;

    let mut facing = North;

    let mut seen : Vec<(usize, usize)> = vec![];

    p.output_fn(move |output| {
        //seen.push(position);
        if count % 2 == 0 {
            canvas.entry(position).insert(output);
        } else {
            if output == 0 {
                facing = facing.rotateAntiClockwise();
            } else {
                facing = facing.rotateClockwise();
            }
            match facing {
                North => position = (position.0, position.1-1),
                East => position = (position.0+1, position.1),
                South => position = (position.0, position.1+1),
                West => position = (position.0-1, position.1),
            }
        }

        count += 1;
    });

    p.exec();

    let count = seen.iter().unique().count();

    eprintln!("count = {:#?}", count);

}
