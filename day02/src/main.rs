use std::fs;
use std::error::Error;

fn main() {
    let input : Vec<i32> = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file")
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let result = part01(&mut input.clone());
    eprintln!("part01 = {:#?}", result);

    part02(&mut input.clone());
        
}

type OpCodes = Vec<i32>;

fn part01(op_codes : &mut OpCodes) -> i32{
    exec_with_init(op_codes, 12, 2).unwrap_or_default()
}

fn part02(op_codes : &mut OpCodes) {
    
    for noun in 1..=100 {
        for verb in 1..=100 {
            let mut new_opcode = op_codes.clone();
            let new_output = exec_with_init(&mut new_opcode, noun, verb);
            if let Ok(19690720) = new_output {
                eprintln!("{},{}", noun, verb);
                break;
            }
        }
    }

}

fn exec_with_init(op_codes : &mut OpCodes, noun: i32, verb: i32) -> Result<i32, Box<dyn Error>> {
    op_codes[1] = noun;
    op_codes[2] = verb;
    exec(op_codes)
}

fn exec(op_codes : &mut OpCodes) -> Result<i32, Box<dyn Error>> {
    let mut p = 0;
    
    while op_codes[p] != 99 {
        let result_reg = op_codes[p+3] as usize;
        let reg_a = op_codes[p+1] as usize;
        let reg_b = op_codes[p+2] as usize;
        match op_codes[p] {
            1 =>  op_codes[result_reg] = op_codes[reg_a] + op_codes[reg_b],
            2 =>  op_codes[result_reg] = op_codes[reg_a] * op_codes[reg_b],
            99 => break,
            _ => return Err(format!("Unknown OpCode {}", op_codes[p]).into())
        }
        
        p+=4;
         
    }
    
    Ok(op_codes[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn some_tests() {
        let mut input = vec![1,0,0,0,99];
        assert_eq!(exec(&mut input).unwrap(), 2);

        let mut input = vec![2,3,0,3,99];
        assert_eq!(exec(&mut input).unwrap(), 2);


        let mut input = vec![2,4,4,5,99,0];
        assert_eq!(exec(&mut input).unwrap(), 2);

        let mut input = vec![1,1,1,4,99,5,6,0,99];
        assert_eq!(exec(&mut input).unwrap(), 30);
    }
}

