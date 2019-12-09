use std::fs;
use std::error::Error;
use crate::ParameterMode::{Immediate, Position};

fn main() {
    let input : Vec<i32> = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut p = Program::new(input, 5);
    p.exec();
        
}

type OpCodes = Vec<i32>;

struct Program {
    op_codes : OpCodes,
    p : usize,
    input: i32
}

#[derive(PartialEq, Debug)]
enum ParameterMode {
    Position,
    Immediate
}

const HALT : i32 = 99;
const ADD : i32 = 1;
const MUL : i32 = 2;
const INPUT : i32 = 3;
const OUTPUT : i32 = 4;
const JUMP_TRUE : i32 = 5;
const JUMP_FALSE : i32 = 6;
const LESS_THAN : i32 = 7;
const EQUALS : i32 = 8;


impl Program {
    fn new(op_codes: OpCodes, input: i32) -> Program {
        Program { p: 0, input, op_codes}
    }
    
    fn exec(&mut self) -> Result<i32, Box<dyn Error>> {
        while self.next_code() != HALT {
            
            let code = self.next_code();
            match code {
                ADD   => self.add(),
                MUL   => self.mul(),
                INPUT => self.input(),
                OUTPUT => self.output(),
                JUMP_TRUE => self.jump_if_true(),
                JUMP_FALSE => self.jump_if_false(),
                LESS_THAN => self.less_than(),
                EQUALS => self.equals(),
                HALT  => break,
                _ => return Err(format!("Unknown OpCode").into())
            }   
        }

        Ok(0)
    }
    
    fn input(&mut self) {
        let index = self.op_codes[self.p + 1] as usize;
        self.op_codes[index] = self.input;
        self.p+=2
    }
    
    fn output(&mut self) {
        println!("{}", self.op_codes[self.op_codes[self.p +1] as usize]);
        self.p+=2
    }
    
    fn jump_if_true(&mut self) {
        if self.param1() !=0 {
            self.p = self.param2() as usize;
        } else {
            self.p +=3;
        }
    }

    fn jump_if_false(&mut self) {
        if self.param1() == 0 {
            self.p = self.param2() as usize;
        } else {
            self.p += 3;
        }
    }

    fn equals(&mut self) {
        self.binary_op(|a, b| if a == b { 1 } else { 0 });
    }
    
    fn less_than(&mut self) {
        self.binary_op(|a, b| if a < b { 1 } else { 0 });
    }
    
    fn add(&mut self) {
        self.binary_op(|a, b| a + b);
    }

    fn mul(&mut self) {
        self.binary_op(|a, b| a * b);
    }

    fn binary_op<F>(&mut self, f: F) 
        where F: Fn(i32, i32) -> i32 {
        let result_reg = self.op_codes[self.p+3] as usize;
        self.op_codes[result_reg] = f(self.param1(), self.param2());
        self.p+=4;
    }
    
    fn next_code(&mut self) -> i32 {
        let (op_code, _ ,_ ,_ ) = Self::decode(self.op_codes[self.p]);
        op_code
    }
    
    fn decode(op_code: i32) -> (i32, ParameterMode, ParameterMode, ParameterMode) {
        let data = format!("{:05}", op_code)
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();
        
        let operation : i32  = format!("{}{}", &data[3], &data[4]).parse().unwrap();
        let p1 = if data[2] == 0 { Position } else { Immediate }; 
        let p2 = if data[1] == 0 { Position } else { Immediate }; 
        let p3 = if data[0] == 0 { Position } else { Immediate }; 
        
        (operation as i32 ,p1,p2,p3)
    }
    
    fn param1(&self) -> i32 {
        let (_,p1,_,_) = Self::decode(self.op_codes[self.p]);
        let (p, op_codes) = (self.p, &self.op_codes);
        if p1 == Position { op_codes[op_codes[p+1] as usize] } else { op_codes[p+1] }
    }

    fn param2(&self) -> i32 {
        let (_,_,p2,_) = Self::decode(self.op_codes[self.p]);
        let (p, op_codes) = (self.p, &self.op_codes);
        if p2 == Position { op_codes[op_codes[p+2] as usize] } else { op_codes[p+2] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn decode() {
        let (code, op1, op2, op3) = Program::decode(1002);
        assert_eq!(code, 2);
        assert_eq!(op1, Position);
        assert_eq!(op2, Immediate);
        assert_eq!(op3, Position);
    }
    
    #[test]
    fn program() {
        let mut program = Program::new(vec![1002, 4, 3, 4, 33], 1);
        program.exec();
    }
}

