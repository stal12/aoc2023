use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 1: {}\n", part1(input_str));
}

fn part1(input: &str) -> String {
    let (commands, nodes, start, end) = parse_input(input);

    let mut current = start;
    let mut command_pos = 0;
    let mut steps = 0;
    while current != end {
        current = nodes[current][commands[command_pos]];

        command_pos += 1;
        if command_pos == commands.len() {
            command_pos = 0;
        }
        steps += 1;
    }

    steps.to_string()
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<[usize; 2]>, usize, usize) {
    let mut input_split = input.split("\n\n");
    let command_line = input_split.next().unwrap();
    let commands: Vec<usize> = command_line.chars()
        .map(|command| match command {
            'L' => 0,
            'R' => 1,
            _ => panic!()
        })
        .collect();

    let nodes_str = input_split.next().unwrap();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut nodes_intermediate: Vec<(&str, &str, &str)> = Vec::new();
    let mut str_number_map = HashMap::new();
    for (i, node_str) in nodes_str.lines().enumerate() {
        let captures = re.captures(node_str).unwrap();
        let mut captures_iter = captures.iter();
        let node_name = captures_iter.nth(1).unwrap().unwrap().as_str();
        let node_left = captures_iter.next().unwrap().unwrap().as_str();
        let node_right = captures_iter.next().unwrap().unwrap().as_str();
        str_number_map.insert(node_name, i);
        nodes_intermediate.push((node_name, node_left, node_right));
    }

    let nodes: Vec<[usize; 2]> = nodes_intermediate.iter()
        .map(|(_, left, right)| [str_number_map[left], str_number_map[right]])
        .collect();

    (commands, nodes, str_number_map["AAA"], str_number_map["ZZZ"])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, "2");
    }
}