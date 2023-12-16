use nalgebra::{DMatrix, Dyn, OMatrix};

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 2: {}\n", part2(input_str));
}

fn part2(input: &str) -> String {
    let matrix = parse_input(input);
    print_char_matrix(&matrix);
    let max_energy = try_all(&matrix);
    max_energy.to_string()
}

fn try_all(matrix: &DMatrix<u8>) -> i32 {
    let mut max_energy = 0;
    for c in 0..matrix.ncols() {
        max_energy = max_energy.max(try_direction(&matrix, (0, c as i32), Direction::Down));
        max_energy = max_energy.max(try_direction(&matrix, (matrix.nrows() as i32 - 1 , c as i32), Direction::Up));
    }
    for r in 0..matrix.nrows() {
        max_energy = max_energy.max(try_direction(&matrix, (r as i32, 0), Direction::Right));
        max_energy = max_energy.max(try_direction(&matrix, (r as i32, matrix.ncols() as i32 - 1), Direction::Left));
    }
    max_energy
}

fn try_direction(matrix: &DMatrix<u8>, pos: (i32, i32), dir: Direction) -> i32 {
    let mut passed_mat = DMatrix::from_element(matrix.nrows(), matrix.ncols(), [false, false, false, false]);
    let mut rays = vec![(pos, dir)];
    while !rays.is_empty() {
        let ray = rays.pop().unwrap();
        let new_rays = travel(matrix, &mut passed_mat, ray.0, ray.1);
        rays.extend(new_rays);
    }
    let energized = count_energized(&passed_mat);
    energized
}

fn count_energized(passed_mat: &DMatrix<[bool; 4]>) -> i32 {
    let mut count = 0;
    for r in 0..passed_mat.nrows() {
        for c in 0..passed_mat.ncols() {
            let val = passed_mat[(r, c)];
            count += (val[0] || val[1] || val[2] || val[3]) as i32;
        }
    }
    count
}

fn step_in_dir(pos: (i32, i32), dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0, pos.1 + 1),
    }
}

fn travel(matrix: &DMatrix<u8>, passed_mat: &mut DMatrix<[bool; 4]>, mut pos: (i32, i32), mut dir: Direction) -> Vec<((i32, i32), Direction)> {
    let mut new_rays = Vec::new();

    while pos.0 >= 0 && pos.0 < matrix.nrows() as i32 && pos.1 >= 0 && pos.1 < matrix.ncols() as i32 && !passed_mat[(pos.0 as usize, pos.1 as usize)][dir as usize] {
        passed_mat[(pos.0 as usize, pos.1 as usize)][dir as usize] = true;

        match matrix[(pos.0 as usize, pos.1 as usize)] as char {
            '.' => (),
            '/' => {
                dir = match dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up
                };
            },
            '\\' => {
                dir = match dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down
                };
            }
            '|' => {
                match dir {
                    Direction::Right | Direction::Left => {
                        passed_mat[(pos.0 as usize, pos.1 as usize)][Direction::Right as usize] = true;
                        passed_mat[(pos.0 as usize, pos.1 as usize)][Direction::Left as usize] = true;
                        dir = Direction::Up;
                        let new_ray_start = step_in_dir(pos, Direction::Down);
                        if new_ray_start.0 < matrix.nrows() as i32 {
                            new_rays.push((new_ray_start, Direction::Down));
                        }
                    }
                    _ => ()
                }
            },
            '-' => {
                match dir {
                    Direction::Up | Direction::Down => {
                        passed_mat[(pos.0 as usize, pos.1 as usize)][Direction::Up as usize] = true;
                        passed_mat[(pos.0 as usize, pos.1 as usize)][Direction::Down as usize] = true;
                        dir = Direction::Left;
                        let new_ray_start = step_in_dir(pos, Direction::Right);
                        if new_ray_start.1 < matrix.ncols() as i32 {
                            new_rays.push((new_ray_start, Direction::Right));
                        }
                    },
                    _ => ()
                }
            }
            _ => panic!()
        }
        pos = step_in_dir(pos, dir);
    }

    new_rays
}

#[repr(usize)]
#[derive(Clone, Copy)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

fn print_char_matrix(matrix: &DMatrix<u8>) {
    for r in 0..matrix.nrows() {
        for c in 0..matrix.ncols() {
            print!("{}", matrix[(r, c)] as char);
        }
        print!("\n");
    }
    print!("\n");
}

fn parse_matrix(input: &str) -> OMatrix<u8, Dyn, Dyn> {
    let nrows = input.lines().count();
    let ncols = input.lines().next().unwrap().bytes().len();
    DMatrix::from_row_iterator(nrows, ncols, input.bytes().filter(|&x| x != '\n' as u8))
}

fn parse_input(input: &str) -> OMatrix<u8, Dyn, Dyn> {
    parse_matrix(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
");
        assert_eq!(result, "51");
    }
}