use regex::Regex;
use std::collections::HashMap;
use num::integer::lcm;

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 2: {}\n", part2(input_str));
}

fn exists_cyle(current_node: usize, steps: usize, zs: &[(usize, usize)], cmd_len: usize) -> Option<(Vec<usize>, usize)> {
    for (i, (z, z_steps)) in zs.iter().copied().enumerate() {
        if z == current_node && (steps - z_steps) % cmd_len == 0 {
            return Some((zs.iter().copied().skip(i).map(|(_, z_steps)| z_steps).collect(), steps - z_steps));
        }
    }
    None
}

fn find_cycle(start_node: usize, end_nodes: &[usize], commands: &[usize], nodes: &[[usize; 2]]) -> (Vec<usize>, usize) {
    let mut zs = Vec::new();
    let mut current_node = start_node;
    let mut command_pos = 0;
    let mut steps = 0;

    loop {
        if end_nodes.contains(&current_node) {
            let optional_cycle = exists_cyle(current_node, steps, &zs, commands.len());
            if optional_cycle.is_some() {
                return optional_cycle.unwrap();
            }
            else {
                zs.push((current_node, steps));
            }
        }

        current_node = nodes[current_node][commands[command_pos]];

        command_pos += 1;
        if command_pos == commands.len() {
            command_pos = 0;
        }
        steps += 1;
    }
}

fn part2(input: &str) -> String {
    let (commands, nodes, start_nodes, end_nodes) = parse_input(input);
    let cycles: Vec<_> = start_nodes.iter().copied()
        .map(|start_node| find_cycle(start_node, &end_nodes, &commands, &nodes))
        .collect();
    print!("Cycles: {:?}\n", cycles);

    cycles.iter().fold(1, |acc, (_, steps)| lcm(acc, *steps)).to_string()
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<[usize; 2]>, Vec<usize>, Vec<usize>) {
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
    let mut start_nodes = Vec::new();
    let mut end_nodes = Vec::new();
    for (i, node_str) in nodes_str.lines().enumerate() {
        let captures = re.captures(node_str).unwrap();
        let mut captures_iter = captures.iter();
        let node_name = captures_iter.nth(1).unwrap().unwrap().as_str();
        let node_left = captures_iter.next().unwrap().unwrap().as_str();
        let node_right = captures_iter.next().unwrap().unwrap().as_str();
        str_number_map.insert(node_name, i);
        nodes_intermediate.push((node_name, node_left, node_right));
        if node_name.ends_with("A") {
            start_nodes.push(i);
        }
        if node_name.ends_with("Z") {
            end_nodes.push(i);
        }
    }

    let nodes: Vec<[usize; 2]> = nodes_intermediate.iter()
        .map(|(_, left, right)| [str_number_map[left], str_number_map[right]])
        .collect();

    (commands, nodes, start_nodes, end_nodes)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");
        assert_eq!(result, "6");
    }
}