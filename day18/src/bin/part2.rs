use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn command_to_delta(command: (Direction, i64)) -> (i64, i64) {
    match command.0 {
        Direction::Up => (-command.1, 0),
        Direction::Down => (command.1, 0),
        Direction::Left => (0, -command.1),
        Direction::Right => (0, command.1),
    }
}

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 2: {}\n", part2(input_str));
}

fn part2(input: &str) -> String {
    let commands = parse_input(input);
    println!("{:?}", commands);
    let polygon = find_polygon(&commands, (0, 0));
    println!("{:?}", polygon);
    let internal_area = shoelace(&polygon);
    let frame_area = find_frame(&commands);
    let area = internal_area + frame_area;
    area.to_string()
}

fn find_polygon(commands: &[(Direction, i64)], start: (i64, i64)) -> Vec<(i64, i64)> {
    let mut polygon: Vec<(i64, i64)> = vec![start];
    let mut pos = start;
    for &command in commands.iter().rev().skip(1).rev() {
        let delta = command_to_delta(command);
        pos.0 += delta.0;
        pos.1 += delta.1;
        polygon.push(pos);
    }
    polygon
}

fn find_frame(commands: &[(Direction, i64)]) -> i64 {
    let mut sum = 0;
    let mut last_dir = commands.last().unwrap().0;
    for &(direction, length) in commands.iter() {
        sum += 2 * (length - 1);
        sum += match (last_dir, direction) {
            (Direction::Up, Direction::Right) | (Direction::Right, Direction::Down) | (Direction::Down, Direction::Left) | (Direction::Left, Direction::Up) => 3,
            _ => 1
        };
        last_dir = direction;
    }
    sum / 4
}

fn shoelace(polygon: &[(i64, i64)]) -> i64 {
    let mut area = 0;
    for (&a, &b) in polygon.iter().zip(polygon.iter().cycle().skip(1)) {
        area += a.0 * b.1 - a.1 * b.0;
    }
    area.abs() / 2
}

fn parse_command(input: &str) -> (Direction, i64) {
    let re = Regex::new(r"\w{6}").unwrap();
    let color = re.find(input).unwrap().as_str().as_bytes();
    let length_str = std::str::from_utf8(&color[0..5]).unwrap();
    let length = i64::from_str_radix(length_str, 16).unwrap();
    let direction = match color[5] as char {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => panic!()
    };
    (direction, length)
}

fn parse_input(input: &str) -> Vec<(Direction, i64)> {
    input.lines().map(|line| parse_command(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
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
        assert_eq!(result, "952408144115");
    }
}