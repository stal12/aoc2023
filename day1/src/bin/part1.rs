use regex::Regex;

fn match_to_digit(m: &str) -> i32 {
    if m.len() == 1 {
        return m.parse().unwrap();
    } else {
        return match m {
            "zero" => 0,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => panic!("unknown digit"),
        };
    }
}

fn main() {
    
    let input_str = include_str!("input.txt");

    print!("Part 1: {}\n", part1(input_str));

    let re_first = Regex::new(r"\d|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let re_last = Regex::new(r".*(\d|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut sum = 0;
    for line in input_str.lines() {
        let first_match = re_first.find(line).unwrap().as_str();
        let last_match = re_last.captures(line).unwrap().get(1).unwrap().as_str();
                
        let first_digit : i32 = match_to_digit(first_match);
        let last_digit : i32 = match_to_digit(last_match);
        
        let value = first_digit * 10 + last_digit;              
        sum += value;
    }

    print!("Part 2: {}\n", sum);


}

fn part1(input: &str) -> String {
    let re = Regex::new(r"\d").unwrap();
    
    let mut sum = 0;
    for line in input.lines() {
        let mut matches = re.find_iter(line).peekable();
        let first_match = matches.peek().unwrap().clone();
        let last_match = matches.last().unwrap().clone();
        
        let first_digit : i32 = match_to_digit(first_match.as_str());
        let last_digit : i32 = match_to_digit(last_match.as_str());
        
        let value = first_digit * 10 + last_digit;              
        sum += value;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        assert_eq!(result, "142");
    }
}