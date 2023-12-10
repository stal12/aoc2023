use nalgebra::{DMatrix, Dyn, OMatrix};

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 2: {}\n", part2(input_str));
}

fn part2(input: &str) -> String {
    let matrix = parse_input(input);
    print_char_matrix(&matrix);
    let s_pos = find_start(&matrix).unwrap();
    print!("{:?}\n", s_pos);
    let (dir_out, dir_in) = find_loop(&matrix, s_pos).unwrap();
    let real_s = find_real_s(dir_out, dir_in);

    let fence_mat = create_fence_mat(&matrix, real_s, next_pos(s_pos, dir_out), dir_out);
    print_char_matrix(&fence_mat);

    let mut sum = 0;
    for r in 0..fence_mat.nrows() {
        let count = count_internal_per_line(&fence_mat, r);
        sum += count;
    }

    sum.to_string()
}

fn count_internal_per_line(fence_mat: &DMatrix<u8>, row: usize) -> usize {
    let mut inside = 0;
    let mut prev_junction = '.' as u8;
    let mut count = 0;
    for c in 0..fence_mat.ncols() {
        match fence_mat[(row, c)] as char {
            '.' => count += inside,
            '|' => inside = 1 - inside,
            'F' => prev_junction = 'F' as u8,
            'L' => prev_junction = 'L' as u8,
            'J' => match prev_junction as char {
                'F' => inside = 1 - inside,
                _ => (),
            }
            '7' => match prev_junction as char {
                'L' => inside = 1 - inside,
                _ => (),
            }
            _ => (),
        }
    }
    count
}

fn print_char_matrix(matrix: &DMatrix<u8>) {
    for r in 0..matrix.nrows() {
        for c in 0..matrix.ncols() {
            print!("{}", matrix[(r, c)] as char);
        }
        print!("\n");
    }
}

fn create_fence_mat(matrix: &DMatrix<u8>, real_s: u8, start: (usize, usize), mut dir: Direction) -> DMatrix<u8> {
    let mut fence_mat = DMatrix::repeat(matrix.nrows(), matrix.ncols(), '.' as u8);
    let mut pos = start;
    while matrix[pos] != 'S' as u8 {
        fence_mat[pos] = matrix[pos];
        dir = next_direction(dir, matrix[pos]).unwrap();
        pos = next_pos(pos, dir);
    }
    fence_mat[pos] = real_s;
    fence_mat
}

fn find_real_s(dir_out: Direction, dir_in: Direction) -> u8 {
    match (dir_out, dir_in) {
        (Direction::Up, Direction::Up) | (Direction::Down, Direction::Down) => '|' as u8,
        (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right) => '-' as u8,
        (Direction::Right, Direction::Down) | (Direction::Up, Direction::Left) => 'L' as u8,
        (Direction::Left, Direction::Down) | (Direction::Up, Direction::Right) => 'J' as u8,
        (Direction::Left, Direction::Up) | (Direction::Down, Direction::Right) => '7' as u8,
        (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => 'F' as u8,
        _ => panic!()
    }
}

fn find_loop(matrix: &DMatrix<u8>, start: (usize, usize)) -> Option<(Direction, Direction)> {
    for dir_out in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
        // There is a bug here, if S is close to a border
        let dir_in = try_follow_loop(&matrix, next_pos(start, dir_out), dir_out);
        if dir_in.is_some() {
            return Some((dir_out, dir_in.unwrap()));
        }
    }
    None
}

fn find_start(matrix: &DMatrix<u8>) -> Option<(usize, usize)> {
    for r in 0..matrix.nrows() {
        for c in 0..matrix.ncols() {
            if matrix[(r, c)] == 'S' as u8 {
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
    Right,
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
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0, pos.1 + 1),
    }
}

fn try_follow_loop(matrix: &DMatrix<u8>, mut pos: (usize, usize), mut dir: Direction) -> Option<Direction> {
    loop {
        if pos.0 >= matrix.nrows() || pos.1 >= matrix.ncols() {
            return None;
        }

        if matrix[pos] as char == 'S' {
            return Some(dir);
        }

        dir = next_direction(dir, matrix[pos])?;
        pos = next_pos(pos, dir);
    };
}

fn parse_input(input: &str) -> OMatrix<u8, Dyn, Dyn> {
    let bytes_input: Vec<u8> = input.as_bytes().iter().copied()
        .filter(|&byte| byte != u8::try_from('\n').unwrap())
        .collect();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let matrix = DMatrix::from_row_slice(height, width, &bytes_input);
    matrix
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
");
        assert_eq!(result, "4");
    }

    #[test]
    fn it_works_2() {
        let result = part2(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
");
        assert_eq!(result, "8");
    }

    #[test]
    fn it_works_3() {
        let result = part2(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
");
        assert_eq!(result, "10");
    }
}