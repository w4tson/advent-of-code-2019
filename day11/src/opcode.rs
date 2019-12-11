use std::error::Error;
use crate::opcode::ParameterMode::{Position, Immediate, Relative};
use std::collections::HashMap;

type OpCodes = Vec<i64>;

pub struct Program {
    op_codes : OpCodes,
    p : usize,
    output: i64,
    input: i64, 
    base_offset: i64,
    memory: HashMap<usize, i64>,
    input_fn: Option<Box<dyn Fn() -> i64>>,
    output_fn: Option<Box<dyn FnMut(i64) -> ()>>,
}

#[derive(PartialEq, Debug)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative
}

impl From<u32> for ParameterMode {
    fn from(i: u32) -> Self {
        match i {
            0 => Position,
            1 => Immediate,
            2 => Relative,
            _ => panic!("Unknown parameter mode")
        }
    }
}

const HALT : i64 = 99;
const ADD : i64 = 1;
const MUL : i64 = 2;
const INPUT : i64 = 3;
const OUTPUT : i64 = 4;
const JUMP_TRUE : i64 = 5;
const JUMP_FALSE : i64 = 6;
const LESS_THAN : i64 = 7;
const EQUALS : i64 = 8;
const ADJUST_BASE_OFFSET : i64 = 9;


impl Program {
    pub fn new(op_codes: &OpCodes) -> Program {
        let op_codes = op_codes.clone();
        let memory : HashMap<usize,i64> = op_codes.iter()
            .enumerate()
            .fold(HashMap::new(), |acc, (i, op_code)| {
                let mut acc = acc;
                acc.entry(i).or_insert(*op_code);
                acc
            });
        
        Program {  p: 0, op_codes, output: 0, input:2,  base_offset: 0, memory, input_fn: None, output_fn: None }
    }

    pub fn input_supplier<F>(&mut self, f: F )
        where F: Fn() -> i64 + 'static {
        self.input_fn = Some(Box::new(f));
    }

    pub fn output_fn<F>(&mut self, f: F )
        where F: FnMut(i64) -> () + 'static {
        self.output_fn = Some(Box::new(f));
    }
    
     pub fn exec(&mut self) -> Result<i64, Box<dyn Error>> {
        while self.next_code() != HALT {

            let code = self.next_code();
            match code {
                ADD   => self.add(),
                MUL   => self.mul(),
                INPUT =>  self.input(),
                OUTPUT => self.output(),
                JUMP_TRUE => self.jump_if_true(),
                JUMP_FALSE => self.jump_if_false(),
                LESS_THAN => self.less_than(),
                EQUALS => self.equals(),
                ADJUST_BASE_OFFSET => self.adj_base_offset(),
                HALT  => break,
                _ => return Err(format!("Unknown OpCode").into())
            }
        }

        Ok(self.output)
    }
    
    fn input(&mut self) {
        println!("input");
        if let Some(input_supplier) = &self.input_fn {
            self.update_param(input_supplier(), 1);

        }
        self.p+=2;
    }
    
    fn update_param(&mut self, value: i64, param: usize) {
        let (_,p1,p2,p3) =  Self::decode(self.memory[&self.p]);
        let param_mode = match param {
            1 => p1,
            2 => p2,
            3 => p3,
            _ => panic!("Can't calc param mode")
        };
        let index = self.p + param;
        let literal_value = *self.memory.get(&index).unwrap_or(&0);
        match param_mode {
            Position => self.memory.insert(literal_value as usize, value),
            Relative => self.memory.insert((self.base_offset + literal_value) as usize, value),
            _ => panic!("Problem updating param1, unknown param mode")
        };
    }

    fn output(&mut self) {
        self.output = self.param1();
        if let Some(f) = &self.output_fn {
            f(self.param1());
        }
        println!("Outputting {}",  self.output);
        self.p+=2
    }
    
    fn adj_base_offset(&mut self) {
        self.base_offset += self.param1();
        self.p+=2;
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
        where F: Fn(i64, i64) -> i64 {
        self.update_param(f(self.param1(), self.param2()), 3);
        self.p+=4;
    }

    fn next_code(&mut self) -> i64 {
        let (op_code, _ ,_ ,_ ) = Self::decode(self.memory[&self.p]);
        op_code
    }

    fn decode(op_code: i64) -> (i64, ParameterMode, ParameterMode, ParameterMode) {
        let data = format!("{:05}", op_code)
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        let operation : i64  = format!("{}{}", &data[3], &data[4]).parse().unwrap();
        let p1 = data[2].into();
        let p2 = data[1].into();
        let p3 = data[0].into();

        (operation as i64 ,p1,p2,p3)
    }

    fn param1(&self) -> i64 {
        let (_,p1,_,_) = Self::decode(self.memory[&self.p]);
        self.resolve_param(p1, 1)
    }

    fn param2(&self) -> i64 {
        let (_,_,p2,_) = Self::decode(self.memory[&self.p]);
        self.resolve_param(p2, 2)
    }
    
    fn resolve_param(&self, param_mode: ParameterMode, param_index: usize) -> i64 {
        let (p, memory) = (self.p, &self.memory);
        let index = p + param_index;
        let literal_value = *memory.get(&index).unwrap_or(&0);
        match param_mode {
            Position => *memory.get(&(literal_value as usize)).unwrap_or(&0),
            Immediate => literal_value,
            Relative => *memory.get(&((self.base_offset + literal_value) as usize)).unwrap_or(&0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcode::ParameterMode::Relative;

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
        let mut program = Program::new(&vec![1002, 4, 3, 4, 33]);
        program.exec();
    }
    
    #[test]
    fn test_enum() {
        let zero : ParameterMode = 0.into();
        let one : ParameterMode = 1.into();
        let two : ParameterMode = 2.into();

        assert_eq!(zero, Position);
        assert_eq!(one, Immediate);
        assert_eq!(two, Relative);
    }
    
    #[test]
    fn test_base_pointer() {
        let input = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut p = Program::new(&input);
        p.exec();
    }
    
    #[test]
    fn sixteen_digit() {
        let input = vec![1102,34915192,34915192,7,4,7,99,0];
        let mut p = Program::new(&input);
        p.exec();
    }
    
    #[test]
    fn number() {
        
        let input = vec![104,1125899906842624,99];
        let mut p = Program::new(&input);
        p.exec();
        
    }
}

