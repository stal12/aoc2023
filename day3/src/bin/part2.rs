use regex::Regex;

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 2: {}\n", part2(input_str));
}

fn part2(input: &str) -> String {
    let schema = parse_input(input);
    let number_re = Regex::new(r"\d+").unwrap();
    let symbol_re = Regex::new(r"[^\d\.]").unwrap();
    let mut sum = 0;
    for (i, line) in schema.iter().enumerate() {
        for symbol_match in symbol_re.find_iter(line) {
            let mut numbers = 0;
            let mut ratio = 1;
            let symbol_pos = symbol_match.start();
            for number_match in number_re.find_iter(line) {
                check_number_match(symbol_pos, &number_match, &mut numbers, &mut ratio)
            }
            if i > 0 {
                for number_match in number_re.find_iter(schema[i - 1]) {
                    check_number_match(symbol_pos, &number_match, &mut numbers, &mut ratio)
                }
            }
            if i + 1 < schema.len() {
                for number_match in number_re.find_iter(schema[i + 1]) {
                    check_number_match(symbol_pos, &number_match, &mut numbers, &mut ratio)
                }
            }
            if numbers == 2 {
                sum += ratio;
            }
        }
    }
    sum.to_string()
}

fn check_number_match(symbol_pos: usize, number_match: &regex::Match, numbers: &mut i32, ratio: &mut i32) {
    if symbol_pos >= number_match.start().saturating_sub(1) &&
        symbol_pos <= number_match.end() {
        *numbers += 1;
        if *numbers <= 2 {
            *ratio *= number_match.as_str().parse::<i32>().unwrap();
        }
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
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
        assert_eq!(result, "467835");
    }
}