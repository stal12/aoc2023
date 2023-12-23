use regex::Regex;
use nalgebra::DMatrix;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn command_to_delta(command: (Direction, i32)) -> (i32, i32) {
    match command.0 {
        Direction::Up => (-command.1, 0),
        Direction::Down => (command.1, 0),
        Direction::Left => (0, -command.1),
        Direction::Right => (0, command.1),
    }
}

fn direction_to_delta(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 1: {}\n", part1(input_str));
}

fn part1(input: &str) -> String {
    let commands = parse_input(input);
    println!("{:?}", commands);
    let (min_r, max_r, min_c, max_c) = find_extremes(&commands);
    let nrows = max_r - min_r + 1;
    let ncols = max_c - min_c + 1;
    let mut matrix: DMatrix<i32> = DMatrix::zeros(nrows as usize, ncols as usize);
    add_border_to_matrix(&mut matrix, &commands, (-min_r, -min_c));
    print_digit_matrix(&matrix);
    let start_pos = (matrix.nrows() / 2, matrix.ncols() / 2);
    flood_fill(&mut matrix, start_pos);
    print_digit_matrix(&matrix);
    count_ones(&matrix).to_string()
}

fn add_border_to_matrix(matrix: &mut DMatrix<i32>, commands: &[(Direction, i32)], start: (i32, i32)) {
    let mut pos = start;
    matrix[(pos.0 as usize, pos.1 as usize)] = 1;
    for &(direction, length) in commands.iter() {
        let delta = direction_to_delta(direction);
        for _ in 0..length {
            pos.0 += delta.0;
            pos.1 += delta.1;
            matrix[(pos.0 as usize, pos.1 as usize)] = 1;
        }
    }
}

fn count_ones(matrix: &DMatrix<i32>) -> i32 {
    let mut count = 0;
    for c in 0..matrix.ncols() {
        for r in 0..matrix.nrows() {
            count += matrix[(r, c)];
        }
    }
    count
}

fn print_digit_matrix(matrix: &DMatrix<i32>) {
    for r in 0..matrix.nrows() {
        for c in 0..matrix.ncols() {
            print!("{}", matrix[(r, c)]);
        }
        print!("\n");
    }
    print!("\n");
}

fn flood_fill(matrix: &mut DMatrix<i32>, start: (usize, usize)) {
    let mut queue: Vec<(usize, usize)> = vec![start];
    while !queue.is_empty() {
        let pos = queue.pop().unwrap();
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        if pos.0 > 0 {
            neighbors.push((pos.0 - 1, pos.1));
        }
        if pos.0 < matrix.nrows() - 1 {
            neighbors.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            neighbors.push((pos.0, pos.1 - 1));
        }
        if pos.1 < matrix.ncols() - 1 {
            neighbors.push((pos.0, pos.1 + 1));
        }
        for neighbor in neighbors {
            if matrix[neighbor] == 0 {
                queue.push(neighbor);
                matrix[neighbor] = 1;
            }
        }
    }
}

fn find_extremes(commands: &[(Direction, i32)]) -> (i32, i32, i32, i32) {
    let mut max_r = 0;
    let mut min_r = 0;
    let mut max_c = 0;
    let mut min_c = 0;
    let mut pos = (0, 0);
    for &command in commands.iter() {
        pos.0 += command_to_delta(command).0;
        pos.1 += command_to_delta(command).1;
        min_r = min_r.min(pos.0);
        max_r = max_r.max(pos.0);
        min_c = min_c.min(pos.1);
        max_c = max_c.max(pos.1);
    }
    (min_r, max_r, min_c, max_c)
}

fn parse_command(input: &str) -> (Direction, i32) {
    let direction = match input.chars().next().unwrap() {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!()
    };
    let re = Regex::new(r"\d+").unwrap();
    let length: i32 = re.find(input).unwrap().as_str().parse().unwrap();
    (direction, length)
}

fn parse_input(input: &str) -> Vec<(Direction, i32)> {
    input.lines().map(|line| parse_command(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
");
        assert_eq!(result, "62");
    }
}