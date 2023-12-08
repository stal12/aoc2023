use regex::Regex;

fn main() {    
    let input_str = include_str!("input.txt");
    print!("Part 1: {}\n", part1(input_str));
}

fn process_race(race: (u32, u32)) -> u32 {
    let time = race.0;
    let distance = race.1;
    (1..time).filter(|speed| speed * (time - speed) > distance)
        .count() as u32
}

fn part1(input: &str) -> String {
    let races = parse_input(input);
    print!("{:?}\n", races);
    races.iter().copied()
        .map(|race| process_race(race))
        .product::<u32>()
        .to_string()
}

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let number_re = Regex::new(r"\d+").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let times: Vec<_> = number_re.find_iter(lines[0])
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect();
    let distances: Vec<_> = number_re.find_iter(lines[1])
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect();
    let races: Vec<_> = times.iter().copied().zip(distances).collect();
    races
}


#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200
");
        assert_eq!(result, "288");
    }
}