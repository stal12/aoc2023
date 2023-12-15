use nalgebra::{DMatrix, Dyn, OMatrix};

fn main() {    
    let input_str = include_str!("input.txt");
    print!("Part 1: {}\n", part1(input_str));
}

fn part1(input: &str) -> String {
    let mut matrix = parse_input(input);
    print_char_matrix(&matrix);
    tilt_north(&mut matrix);
    print_char_matrix(&matrix);
    let load = compute_load(&matrix);
    load.to_string()
}

fn move_rock_north(matrix: &mut DMatrix<u8>, row: usize, col: usize) {
    let mut final_row = row;
    for up_row in (0..row).rev() {
        if matrix[(up_row, col)] == '.' as u8 {
            final_row -= 1;
        }
        else {
            break;
        }
    }
    if final_row != row {
        matrix[(row, col)] = '.' as u8;
        matrix[(final_row, col)] = 'O' as u8;
    }
}

fn tilt_row_north(matrix: &mut DMatrix<u8>, row: usize) {
    for col in 0..matrix.ncols() {
        if matrix[(row, col)] == 'O' as u8 {
            move_rock_north(matrix, row, col);
        }
    }
}

fn tilt_north(matrix: &mut DMatrix<u8>) {
    for row in 0..matrix.nrows() {
        tilt_row_north(matrix, row);
    }
}

fn compute_load(matrix: &DMatrix<u8>) -> usize {
    let mut load = 0;
    for r in 0..matrix.nrows() {
        for c in 0..matrix.ncols() {
            if matrix[(r, c)] == 'O' as u8 {
                load += matrix.nrows() - r;
            }
        }
    }
    load
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
        let result = part1(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
");
        assert_eq!(result, "136");
    }
}