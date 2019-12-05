use std::fs;
use std::error::Error;
use crate::ParameterMode::{Immediate, Position};

fn main() {
    let input : Vec<i32> = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut p = Program::new(input);
    p.exec();
        
}

type OpCodes = Vec<i32>;

struct Program {
    op_codes : OpCodes,
    p : usize
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


impl Program {
    fn new(op_codes: OpCodes) -> Program {
        Program { p: 0, op_codes}
    }
    
    fn exec(&mut self) -> Result<i32, Box<dyn Error>> {
        while self.next_code() != HALT {
            
            let code = self.next_code();
            match code {
                ADD   => self.add(),
                MUL   => self.mul(),
                INPUT => self.input(),
                OUTPUT => self.output(),
                HALT  => break,
                _ => return Err(format!("Unknown OpCode").into())
            }   
        }

        Ok(0)
    }
    
    fn input(&mut self) {
        let index = self.op_codes[self.p + 1] as usize;

        self.op_codes[index] = 1;
        self.p+=2
    }
    
    fn output(&mut self) {
        println!("{}", self.op_codes[self.op_codes[self.p +1] as usize]);
        self.p+=2
    }
    
    fn binary_op<F>(&mut self, f: F) 
        where F: Fn(i32, i32) -> i32 {
        let (_, p1 , p2 , _ ) = Self::decode(self.op_codes[self.p]);
        let op_codes = &mut self.op_codes;
        let p = self.p;
        let result_reg = op_codes[p+3] as usize;
        let value_a = if p1 == Position { op_codes[op_codes[p+1] as usize] } else { op_codes[p+1] };
        let value_b = if p2 == Position { op_codes[op_codes[p+2] as usize] } else { op_codes[p+2] };

        op_codes[result_reg] = f(value_a, value_b);
        self.p+=4;
    }
    
    fn add(&mut self) {
        self.binary_op(|a, b| a + b);
    }

    fn mul(&mut self) {
        self.binary_op(|a, b| a * b);
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
        let mut program = Program::new(vec![1002, 4, 3, 4, 33]);
        program.exec();
    }
}

