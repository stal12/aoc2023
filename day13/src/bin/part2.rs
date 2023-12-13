use nalgebra::{DMatrix, Dyn, OMatrix};

fn main() {
    let input_str = include_str!("input.txt");
    print!("part 2: {}\n", part2(input_str));
}

fn part2(input: &str) -> String {
    let matrices = parse_input(input);
    for matrix in matrices.iter() {
        print_char_matrix(matrix);
        print!("{}\n", find_symmetry(&matrix));
    }
    matrices.iter().map(|matrix| find_symmetry(matrix)).sum::<usize>().to_string()
}

fn is_symmetric_after_column(matrix: &DMatrix<u8>, col: usize) -> bool {
    let mut i = 0;
    let mut count_diff = 0usize;
    loop {
        count_diff += matrix.column(col - i).iter().zip(matrix.column(col + i + 1).iter())
            .fold(0usize, |acc: usize, (&a, &b)| acc + (a!=b) as usize);
        if col - i == 0 || col + i + 1 == matrix.ncols() - 1 {
            break;
        }
        i += 1;
    }
    count_diff == 1
}

fn is_symmetric_after_row(matrix: &DMatrix<u8>, row: usize) -> bool {
    let mut i = 0;
    let mut count_diff = 0usize;
    loop {
        count_diff += matrix.row(row - i).iter().zip(matrix.row(row + i + 1).iter())
            .fold(0usize, |acc: usize, (&a, &b)| acc + (a!=b) as usize);
        if row - i == 0 || row + i + 1 == matrix.nrows() - 1 {
            break;
        }
        i += 1;
    }
    count_diff == 1
}

fn find_symmetry(matrix: &DMatrix<u8>) -> usize {
    for col in 0..matrix.ncols()-1 {
        if is_symmetric_after_column(&matrix, col) {
            return col + 1;
        }
    }
    for row in 0..matrix.nrows()-1 {
        if is_symmetric_after_row(&matrix, row) {
            return (row + 1) * 100;
        }
    }
    panic!()
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

fn parse_input(input: &str) -> Vec<OMatrix<u8, Dyn, Dyn>> {
    input.split("\n\n").map(|block| parse_matrix(block)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
");
        assert_eq!(result, "400");
    }
}