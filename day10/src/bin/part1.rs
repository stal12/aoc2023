use nalgebra::{DMatrix, Dyn, OMatrix};

fn main() {    
    let input_str = include_str!("input.txt");
    print!("Part 1: {}\n", part1(input_str));
}

fn part1(input: &str) -> String {
    let matrix = parse_input(input);
    let s_pos = find_start(&matrix).unwrap();
    let (_dir, steps) = find_loop(&matrix, s_pos).unwrap();
    (steps / 2).to_string()
}

fn find_loop(matrix: &DMatrix<u8>, start: (usize, usize)) -> Option<(Direction, usize)> {
    for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
        let steps = try_follow_loop(&matrix, next_pos(start, dir), dir);
        if steps.is_some() {
            return Some((dir, steps.unwrap()))
        }
    }
    None
}

fn find_start(matrix: &DMatrix<u8>) -> Option<(usize, usize)> {
    for r in 0..matrix.nrows() {
        for c in 0..matrix.ncols() {
            if matrix[(r, c)] == u8::try_from('S').unwrap() {
                return Some((r, c));
            }
        }
    }
    None
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn next_direction(dir: Direction, tile: u8) -> Option<Direction> {
    match tile as char {
        '|' => match dir {
            Direction::Up => Some(Direction::Up),
            Direction::Down => Some(Direction::Down),
            _ => None,
        },
        '-' => match dir {
            Direction::Left => Some(Direction::Left),
            Direction::Right => Some(Direction::Right),
            _ => None,
        },
        'L' => match dir {
            Direction::Down => Some(Direction::Right),
            Direction::Left => Some(Direction::Up),
            _ => None,
        },
        'J' => match dir {
            Direction::Down => Some(Direction::Left),
            Direction::Right => Some(Direction::Up),
            _ => None,
        },
        '7' => match dir {
            Direction::Up => Some(Direction::Left),
            Direction::Right => Some(Direction::Down),
            _ => None,
        },
        'F' => match dir {
            Direction::Up => Some(Direction::Right),
            Direction::Left => Some(Direction::Down),
            _ => None,
        },
        _ => None
    }
}

fn next_pos(pos: (usize, usize), dir: Direction) -> (usize, usize) {
    match dir {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

fn try_follow_loop(matrix: &DMatrix<u8>, mut pos: (usize, usize), mut dir: Direction) -> Option<usize> {
    let mut steps = 1;
    loop {
        if pos.0 >= matrix.ncols() || pos.1 >= matrix.nrows() {
            return None;
        }

        if matrix[pos] as char == 'S' {
            return Some(steps);
        }

        dir = next_direction(dir, matrix[pos])?;
        pos = next_pos(pos, dir);
        steps += 1;
    };
}

fn parse_input(input: &str) -> OMatrix<u8, Dyn, Dyn> {
    let bytes_input: Vec<u8> = input.as_bytes().iter().copied()
        .filter(|&byte| byte != u8::try_from('\n').unwrap())
        .collect();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let matrix = DMatrix::from_vec(height, width, bytes_input);
    matrix
}


#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            ".....
.S-7.
.|.|.
.L-J.
.....
");
        assert_eq!(result, "4");
    }

    #[test]
    fn it_works_2() {
        let result = part1(
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
");
        assert_eq!(result, "4");
    }

    #[test]
    fn it_works_3() {
        let result = part1(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
");
        assert_eq!(result, "8");
    }

    #[test]
    fn it_works_4() {
        let result = part1(
            "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
");
        assert_eq!(result, "8");
    }
}