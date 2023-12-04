use regex::Regex;

fn main() {    
    let input_str = include_str!("input.txt");
    print!("Part 1: {}\n", part1(input_str));
}

fn part1(input: &str) -> String {
    let schema = parse_input(input);
    let line_size = schema[0].len();
    let number_re = Regex::new(r"\d+").unwrap();
    let symbol_re = Regex::new(r"[^\d\.]").unwrap();
    let mut sum = 0;
    for (i, line) in schema.iter().enumerate() {
        for number_match in number_re.find_iter(line) {
            let mut touches_symbol = false;
            let search_start =  number_match.start().saturating_sub(1);
            let search_end = (number_match.end() + 1).min(line_size);

            let chars_around = &line[search_start..search_end];
            if symbol_re.is_match(chars_around) {
                touches_symbol = true;
            }
            if !touches_symbol && i > 0 {
                let upper_line = schema[i - 1];
                let chars_above = &upper_line[search_start..search_end];
                if symbol_re.is_match(chars_above) {
                    touches_symbol = true;
                }
            }
            if !touches_symbol && i + 1 < schema.len() {
                let lower_line = schema[i + 1];
                let chars_below = &lower_line[search_start..search_end];
                if symbol_re.is_match(chars_below) {
                    touches_symbol = true;
                }
            }

            if touches_symbol {
                sum += number_match.as_str().parse::<i32>().unwrap();
            }
        }
    }
    sum.to_string()
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
");
        assert_eq!(result, "4361");
    }
}