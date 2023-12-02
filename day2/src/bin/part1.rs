use regex::Regex;

fn main() {    
    let input_str = include_str!("input.txt");

    print!("Part 1: {}\n", part1(input_str));
}

fn part1(input: &str) -> String {
    let games = parse_input(input);
    let color = [12, 13, 14];
    let mut possible_games = 0;
    for (pos, game) in games.iter().enumerate() {
        if is_game_possible(&game, color) {
            possible_games += pos + 1;
        }
    }
    possible_games.to_string()
}

fn is_game_possible(game: &[[i32; 3]], color: [i32; 3]) -> bool {
    for extraction in game {
        if extraction[0] > color[0] || extraction[1] > color[1] || extraction[2] > color[2] {
            return false;
        }
    }
    true
}

fn parse_input(input: &str) -> Vec<Vec<[i32; 3]>> {
    let mut games = Vec::new();
    for line in input.lines() {
        let mut extractions: Vec<[i32; 3]> = Vec::new();
        let game_desc = line.split(':').nth(1).unwrap();
        for extraction in game_desc.split(';') {
            extractions.push(parse_extraction(extraction));
        }
        games.push(extractions);
    }
    games
}

fn parse_extraction(extraction: &str) -> [i32; 3] {
    let mut color = [0; 3];
    let red_re = Regex::new(r"(\d+) r").unwrap();
    let green_re = Regex::new(r"(\d+) g").unwrap();
    let blue_re = Regex::new(r"(\d+) b").unwrap();
    let red_captures = red_re.captures(extraction);
    let green_captures = green_re.captures(extraction);
    let blue_captures = blue_re.captures(extraction);
    if red_captures.is_some() {
        color[0] = red_captures.unwrap().iter().nth(1).unwrap().unwrap().as_str().parse().unwrap();
    }
    if green_captures.is_some() {
        color[1] = green_captures.unwrap().iter().nth(1).unwrap().unwrap().as_str().parse().unwrap();
    }
    if blue_captures.is_some() {
        color[2] = blue_captures.unwrap().iter().nth(1).unwrap().unwrap().as_str().parse().unwrap();
    }
    color
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "8");
    }
}