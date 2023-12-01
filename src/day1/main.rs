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

    let re = Regex::new(r"\d").unwrap();
    
    let mut sum = 0;
    for line in input_str.lines() {
        let mut matches = re.find_iter(line).peekable();
        let first_match = matches.peek().unwrap().clone();
        let last_match = matches.last().unwrap().clone();
        
        let first_digit : i32 = match_to_digit(first_match.as_str());
        let last_digit : i32 = match_to_digit(last_match.as_str());
        
        let value = first_digit * 10 + last_digit;              
        sum += value;
    }

    print!("Part 1: {}\n", sum);


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
