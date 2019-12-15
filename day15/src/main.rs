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

#[derive(Debug,Clone)]
enum Status {
    Wall, 
    SuccessfulMove,
    FoundOxygen
}

impl From<i32> for Direction {
    fn from(i: i32) -> Self {
        match i {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => panic!("Unknown Direction")
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
        *state.next_move().into()
    });

    p.set_output_fn(|state,output| {
        let mut state = state.clone();
        let position = state.position;
        
        
        
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
    pub fn next_move(&self) -> Direction {
        North
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
