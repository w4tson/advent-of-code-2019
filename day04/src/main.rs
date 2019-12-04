use itertools::Itertools;

fn main() {
    let part1 = solve(meets_criteria01);
    eprintln!("part1 = {:#?}", part1);

    let part2 = solve(meets_criteria02);
    eprintln!("part2 = {:#?}", part2);
}

fn solve<C>(criteria: C) -> usize 
    where C : Fn(&str) -> bool {
    (273025..767253)
        .filter(|password| criteria(&password.to_string()))
        .count()
}


fn meets_criteria01(password: &str) -> bool {
    is_not_decreasing(password) && contains_double(password)
}

fn meets_criteria02(password: &str) -> bool {
    is_not_decreasing(password) && contains_double_no_surrounding(password)
}

fn is_not_decreasing(password: &str) -> bool {
    password.chars()
        .filter_map(|c| c.to_digit(10))
        .tuple_windows()
        .fold(true,|acc, (prev,next)| acc && next >= prev)
}

fn contains_double(password: &str) -> bool {
    password.chars()
        .tuple_windows()
        .any(|(a,b)| a == b)
}

fn contains_double_no_surrounding(password: &str) -> bool {
    let doubles_inside = password.chars()
        .tuple_windows::<(_,_,_,_)>()
        .fold(false, |acc, (a, b, c ,d)| {
            acc || b == c && a != b && c != d
        });

    let start : Vec<char>= password.chars().take(3).collect();
    let (a, b, c) = (&start[0], &start[1], &start[2]);
    let starting_double = a == b && b != c;
    
    let end : Vec<char>= password.chars().rev().take(3).collect();
    let (a, b, c) = (&end[0], &end[1], &end[2]);
    let ending_double = a == b && b != c;

    doubles_inside || starting_double || ending_double
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_not_dec() {
        assert_eq!(is_not_decreasing("123456"), true);
        assert_eq!(is_not_decreasing("123455"), true);
        assert_eq!(is_not_decreasing("123454"), false);
    }
    
    #[test]
    fn doubles() {
        assert_eq!(contains_double("asdf"), false);
        assert_eq!(contains_double("asdff"), true);
    }
    
    #[test]
    fn examples() {
        assert_eq!(meets_criteria01("111111"), true);
        assert_eq!(meets_criteria01("223450"), false);
        assert_eq!(meets_criteria01("123789"), false);
    }
    
    #[test]
    fn doubles_no_surrounding() {
        assert_eq!(contains_double_no_surrounding("111221"), true);
        assert_eq!(contains_double_no_surrounding("1112221"), false);
        assert_eq!(contains_double_no_surrounding("112221"), true);
        assert_eq!(contains_double_no_surrounding("122211"), true);
    }
}