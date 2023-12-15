use nalgebra::{DMatrix, Dyn, OMatrix};

fn main() {
    let input_str = include_str!("input.txt");
    print!("part 2: {}\n", part2(input_str));
}

fn part2(input: &str) -> &str {
    let mut matrix = parse_input(input);
    print_char_matrix(&matrix);
    for i in 0..1000 {
        tilt_cycle(&mut matrix);
        let load = compute_load(&matrix);
        print!("{}:\t{}\n", i, load);
    }
    "The cycle has been found by hand"
}

fn tilt_cycle(matrix: &mut DMatrix<u8>) {
    tilt_north(matrix);
    tilt_west(matrix);
    tilt_south(matrix);
    tilt_east(matrix);
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

fn move_rock_south(matrix: &mut DMatrix<u8>, row: usize, col: usize) {
    let mut final_row = row;
    for down_row in row+1..matrix.nrows() {
        if matrix[(down_row, col)] == '.' as u8 {
            final_row += 1;
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

fn move_rock_west(matrix: &mut DMatrix<u8>, row: usize, col: usize) {
    let mut final_col = col;
    for col_left in (0..col).rev() {
        if matrix[(row, col_left)] == '.' as u8 {
            final_col -= 1;
        }
        else {
            break;
        }
    }
    if final_col != col {
        matrix[(row, col)] = '.' as u8;
        matrix[(row, final_col)] = 'O' as u8;
    }
}

fn move_rock_east(matrix: &mut DMatrix<u8>, row: usize, col: usize) {
    let mut final_col = col;
    for col_right in col+1..matrix.ncols() {
        if matrix[(row, col_right)] == '.' as u8 {
            final_col += 1;
        }
        else {
            break;
        }
    }
    if final_col != col {
        matrix[(row, col)] = '.' as u8;
        matrix[(row, final_col)] = 'O' as u8;
    }
}

fn tilt_row_north(matrix: &mut DMatrix<u8>, row: usize) {
    for col in 0..matrix.ncols() {
        if matrix[(row, col)] == 'O' as u8 {
            move_rock_north(matrix, row, col);
        }
    }
}

fn tilt_row_south(matrix: &mut DMatrix<u8>, row: usize) {
    for col in 0..matrix.ncols() {
        if matrix[(row, col)] == 'O' as u8 {
            move_rock_south(matrix, row, col);
        }
    }
}

fn tilt_col_west(matrix: &mut DMatrix<u8>, col: usize) {
    for row in 0..matrix.nrows() {
        if matrix[(row, col)] == 'O' as u8 {
            move_rock_west(matrix, row, col);
        }
    }
}

fn tilt_col_east(matrix: &mut DMatrix<u8>, col: usize) {
    for row in 0..matrix.nrows() {
        if matrix[(row, col)] == 'O' as u8 {
            move_rock_east(matrix, row, col);
        }
    }
}

fn tilt_north(matrix: &mut DMatrix<u8>) {
    for row in 0..matrix.nrows() {
        tilt_row_north(matrix, row);
    }
}

fn tilt_south(matrix: &mut DMatrix<u8>) {
    for row in (0..matrix.nrows()).rev() {
        tilt_row_south(matrix, row);
    }
}

fn tilt_west(matrix: &mut DMatrix<u8>) {
    for col in 0..matrix.ncols() {
        tilt_col_west(matrix, col);
    }
}

fn tilt_east(matrix: &mut DMatrix<u8>) {
    for col in (0..matrix.ncols()).rev() {
        tilt_col_east(matrix, col);
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
        let result = part2(
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