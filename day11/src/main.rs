#![feature(entry_insert)]
use std::fs;
use crate::opcode::Program;
use std::collections::HashMap;
use crate::Direction::{North, South, West, East};
use itertools::Itertools;



type Coord = (i32, i32);

pub mod opcode;

#[derive(Debug,Clone)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            North => East,
            East=> South,
            South => West,
            West => North
        }
    }

    pub fn rotate_anti_clockwise(&self) -> Direction {
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

    let mut p : Program<State> = Program::new(&op_codes);

    p.set_input_fn(|state| {
        *state.canvas.get(&state.position)
            .unwrap_or(&0)
    });

    p.set_output_fn(|state,output| {
        let mut state = state.clone();
        let position = state.position;
        if state.count % 2 == 0 {
            state.canvas.entry(state.position).insert(output);
            state.seen.push(state.position);

        } else {
            if output == 0 {
                state.facing = state.facing.rotate_anti_clockwise()
            } else {
                state.facing = state.facing.rotate_clockwise()
            };

            state.position = match state.facing {
                North =>  (position.0, position.1-1),
                East =>  (position.0+1, position.1),
                South =>  (position.0, position.1+1),
                West =>  (position.0-1, position.1),
            };

        }
        state.count += 1;
        state
    });

    let final_state = p.exec().expect("bad");

    eprintln!("unique locations painted = {}", final_state.seen.iter().unique().count());

    final_state.write();

}

#[derive(Debug,Clone)]
struct State {
    canvas: HashMap<Coord, i64>,
    position: Coord,
    count: usize,
    facing: Direction,
    seen: Vec<Coord>
}

impl State {
    pub fn write(&self) {
        let (min_x,_) = self.canvas.keys().min_by_key(|(x, _)| *x).unwrap();
        let (max_x,_) = self.canvas.keys().max_by_key(|(x, _)| *x).unwrap();

        let (_, min_y) = self.canvas.keys().min_by_key(|( _, y)| *y).unwrap();
        let (_, max_y) = self.canvas.keys().max_by_key(|( _, y)| *y).unwrap();

        for y in *min_y..=*max_y {
            for x in *min_x..=*max_x {
                match self.canvas.get(&(x,y)).unwrap_or(&0) {
                    0 => print!(" "),
                    _ => print!("#")
                }
            }
                println!();
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let mut canvas: HashMap<Coord, i64> = HashMap::new();
        canvas.insert((0,0), 1);

        State {
           canvas, position: (0,0), count: 0 , facing: North, seen: vec![]
        }
    }
}
