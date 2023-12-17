use nalgebra::{DMatrix, Dyn, OMatrix};

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 2: {}\n", part2(input_str));
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Pos {
    row: usize,
    col: usize
}

fn new_momentum(momentum: usize, diff: (i32, i32)) -> Option<usize> {
    match diff {
        (-1, 0) => match momentum {
            0..=8 => Some(momentum + 1),
            9 => None,
            10..=19 => None,
            20..=22 => None,
            30..=32 => None,
            _ => Some(0)
        },
        (1, 0) => match momentum {
            10..=18 => Some(momentum + 1),
            19 => None,
            0..=9 => None,
            20..=22 => None,
            30..=32 => None,
            _ => Some(10)
        },
        (0, -1) => match momentum {
            20..=28 => Some(momentum + 1),
            29 => None,
            30..=39 => None,
            0..=2 => None,
            10..=12 => None,
            _ => Some(20)
        },
        (0, 1) => match momentum {
            30..=38 => Some(momentum + 1),
            39 => None,
            20..=29 => None,
            0..=2 => None,
            10..=12 => None,
            _ => Some(30)
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

fn part2(input: &str) -> String {
    let matrix = parse_input(input);
    print_u8_matrix(&matrix);

    let mut losses= DMatrix::from_element(matrix.nrows(), matrix.ncols(), [usize::MAX; 40]);
    let mut prev= DMatrix::from_element(matrix.nrows(), matrix.ncols(), [Pos{ row: 0, col: 0 }; 40]);
    let mut visited= DMatrix::from_element(matrix.nrows(), matrix.ncols(), [false; 40]);
    let mut queue: Vec<Vertex> = Vec::new();
    queue.push(Vertex{pos: Pos{row: 0, col: 0}, momentum: 40, loss: 0});
    losses[(0, 0)][0] = 0;

    while !queue.is_empty() {
        let (u_index, _) = queue.iter().enumerate().min_by_key(|(_, &v)| v.loss).unwrap();
        let u = queue.swap_remove(u_index);
        if (u.pos != Pos{row: 0, col: 0}) {
            visited[(u.pos.row, u.pos.col)][u.momentum] = true;
        }

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
            if new_pos.row == matrix.nrows() - 1 && new_pos.col == matrix.ncols() - 1 {
                if nei_momentum % 10 < 3 {
                    break;
                }
            }

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

fn print_u8_matrix(matrix: &DMatrix<u8>) {
    for r in 0..matrix.nrows() {
        for c in 0..matrix.ncols() {
            print!("{}", matrix[(r, c)]);
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
        let result = part2(
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
        assert_eq!(result, "94");
    }

    #[test]
    fn it_works_2() {
        let result = part2(
            r"111111111111
999999999991
999999999991
999999999991
999999999991
");
        assert_eq!(result, "71");
    }
}