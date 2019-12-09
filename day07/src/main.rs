use std::fs;
use std::error::Error;
use crate::opcode::Program;
use itertools::Itertools;


pub mod opcode;

fn main() -> Result<(), Box<&'static str>>{
    let input : Vec<i32> = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let result = part1(&input);
    eprintln!("part1 = {:#?}", result);
    
     Ok(())   
}

fn part1(input : &Vec<i32>) -> Result<i32, Box<&str>>{
    (0..5).permutations(5)
        .map(|setting| try_combo(input, &setting).expect("invalid combo"))
        .max()
        .ok_or(Box::new("No max value"))
}


fn try_combo(input : &Vec<i32>, settings: &[i32]) -> Result<i32, Box<dyn Error>> {
    
    let output = settings.iter()
        .fold(0, |output, &phase| {
            let mut p = Program::new(&input);
            p.pipe(phase, move || output);
            p.exec().expect("Bad result for program")
        });
    
    Ok(output)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let input = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let result = try_combo(&input, &[4, 3, 2, 1, 0]).unwrap();
        assert_eq!(result, 43210);
    }
    
    #[test]
    fn test2() {
        let input = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        let result = try_combo(&input, &[0,1,2,3,4]).unwrap();
        assert_eq!(result, 54321);
    }

    #[test]
    fn test3() {
        let input = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,00];
        let result = try_combo(&input, &[1,0,4,3,2]).unwrap();
        assert_eq!(result, 65210);
    }
}
