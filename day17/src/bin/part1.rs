use nalgebra::{DMatrix, Dyn, OMatrix};

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 1: {}\n", part1(input_str));
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Pos {
    row: usize,
    col: usize
}

fn pos_diff(a: Pos, b: Pos) -> (i32, i32) {
    (a.row as i32 - b.row as i32, a.col as i32 - b.col as i32)
}

fn new_momentum(momentum: usize, diff: (i32, i32)) -> Option<usize> {
    match diff {
        (-1, 0) => match momentum {
        0 | 1 => Some(momentum + 1),
            2 => None,
            3..=5 => None,
            _ => Some(0)
        },
        (1, 0) => match momentum {
            3 | 4 => Some(momentum + 1),
            5 => None,
            0..=2 => None,
            _ => Some(3)
        },
        (0, -1) => match momentum {
            6 | 7 => Some(momentum + 1),
            8 => None,
            9..=11 => None,
            _ => Some(6)
        },
        (0, 1) => match momentum {
            9 | 10 => Some(momentum + 1),
            11 => None,
            6..=8 => None,
            _ => Some(9)
        },
        _ => panic!()
    }
}

#[derive(Clone, Copy, Debug)]
struct Vertex {
    pos: Pos,
    momentum: usize,
    loss: usize
}

fn part1(input: &str) -> String {
    let matrix = parse_input(input);
    print_u8_matrix(&matrix);

    let mut losses= DMatrix::from_element(matrix.nrows(), matrix.ncols(), [usize::MAX; 12]);
    let mut prev= DMatrix::from_element(matrix.nrows(), matrix.ncols(), [Pos{ row: 0, col: 0 }; 12]);
    let mut visited= DMatrix::from_element(matrix.nrows(), matrix.ncols(), [false; 12]);
    let mut queue: Vec<Vertex> = Vec::new();
    queue.push(Vertex{pos: Pos{row: 0, col: 0}, momentum: 0, loss: 0});
    losses[(0, 0)][0] = 0;

    while !queue.is_empty() {
        let (u_index, _) = queue.iter().enumerate().min_by_key(|(_, &v)| v.loss).unwrap();
        let u = queue.swap_remove(u_index);
        visited[(u.pos.row, u.pos.col)][u.momentum] = true;

        let mut neighbors = Vec::new();
        if u.pos.row > 0 {
            let neighbor_momentum = new_momentum(u.momentum, (-1, 0));
            if neighbor_momentum.is_some() {
                neighbors.push((Pos { row: u.pos.row - 1, col: u.pos.col }, neighbor_momentum.unwrap()));
            }
        }
        if u.pos.row < matrix.nrows() - 1 {
            let neighbor_momentum = new_momentum(u.momentum, (1, 0));
            if neighbor_momentum.is_some() {
                neighbors.push((Pos { row: u.pos.row + 1, col: u.pos.col }, neighbor_momentum.unwrap()));
            }
        }
        if u.pos.col > 0 {
            let neighbor_momentum = new_momentum(u.momentum, (0, -1));
            if neighbor_momentum.is_some() {
                neighbors.push((Pos { row: u.pos.row, col: u.pos.col - 1}, neighbor_momentum.unwrap()));
            }
        }
        if u.pos.col < matrix.ncols() - 1 {
            let neighbor_momentum = new_momentum(u.momentum, (0, 1));
            if neighbor_momentum.is_some() {
                neighbors.push((Pos { row: u.pos.row, col: u.pos.col + 1}, neighbor_momentum.unwrap()));
            }
        }

        for (new_pos, nei_momentum) in neighbors {
            if !visited[(new_pos.row, new_pos.col)][nei_momentum] {
                let alternative_loss = u.loss + matrix[(new_pos.row, new_pos.col)] as usize;
                if alternative_loss < losses[(new_pos.row, new_pos.col)][nei_momentum] {
                    losses[(new_pos.row, new_pos.col)][nei_momentum] = alternative_loss;
                    prev[(new_pos.row, new_pos.col)][nei_momentum] = u.pos;

                    let new_pos_in_queue = queue.iter().position(|&vertex| vertex.pos == new_pos && vertex.momentum == nei_momentum);
                    if new_pos_in_queue.is_some() {
                        queue.swap_remove(new_pos_in_queue.unwrap());
                    }
                    queue.push(Vertex{pos: new_pos, momentum: nei_momentum, loss: alternative_loss});
                }
            }
        }
    }

    println!("{:?}\n", losses[(matrix.nrows() - 1, matrix.ncols() - 1)]);
    losses[(matrix.nrows() - 1, matrix.ncols() - 1)].iter().min().unwrap().to_string()
}

fn print_pos_matrix(matrix: &DMatrix<Pos>) {
    for r in 0..matrix.nrows() {
        for c in 0..matrix.ncols() {
            let pos = matrix[(r, c)];
            print!("({}, {})\t", pos.row, pos.col);
        }
        print!("\n");
    }
    print!("\n");
}

fn print_u8_matrix(matrix: &DMatrix<u8>) {
    for r in 0..matrix.nrows() {
        for c in 0..matrix.ncols() {
            print!("{}", matrix[(r, c)]);
        }
        print!("\n");
    }
    print!("\n");
}

fn print_usize_matrix(matrix: &DMatrix<usize>) {
    for r in 0..matrix.nrows() {
        for c in 0..matrix.ncols() {
            print!("{}\t", matrix[(r, c)]);
        }
        print!("\n");
    }
    print!("\n");
}


fn parse_matrix(input: &str) -> OMatrix<u8, Dyn, Dyn> {
    let nrows = input.lines().count();
    let ncols = input.lines().next().unwrap().bytes().len();
    DMatrix::from_row_iterator(nrows, ncols, input.bytes().filter(|&x| x != '\n' as u8).map(|x| x - '0' as u8))
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
            r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
");
        assert_eq!(result, "102");
    }
}