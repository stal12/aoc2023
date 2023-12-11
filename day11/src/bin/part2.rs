fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 1: {}\n", part1(input_str));
}

fn part1(input: &str) -> String {
    let (galaxies, empty_rows, empty_cols) = parse_input(input);
    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            sum += galaxy_distance(galaxies[i], galaxies[j], &empty_rows, &empty_cols);
        }
    }

    sum.to_string()
}

fn galaxy_distance(a: (usize, usize), b: (usize, usize), empty_rows: &[bool], empty_cols: &[bool]) -> usize {
    let min_r = a.0.min(b.0);
    let max_r = a.0.max(b.0);
    let min_c = a.1.min(b.1);
    let max_c = a.1.max(b.1);
    let void_width = 1000000usize;
    let mut distance = 0usize;
    for r in min_r..max_r {
        distance += match empty_rows[r] {
            true => void_width,
            false => 1
        }
    }
    for c in min_c..max_c {
        distance += match empty_cols[c] {
            true => void_width,
            false => 1
        }
    }
    distance
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<bool>, Vec<bool>) {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    let mut nrows = 0usize;
    let mut ncols = 0usize;
    for (r, line) in input.lines().enumerate() {
        nrows += 1;
        for (c, ch) in line.bytes().enumerate() {
            if c == 0 {
                ncols += 1;
            }
            match ch as char {
                '#' => galaxies.push((r, c)),
                '.' => (),
                _ => panic!()
            }
        }
    }

    let mut empty_rows = vec![true; nrows];
    let mut empty_cols = vec![true; ncols];
    for (galaxy_r, galaxy_c) in galaxies.iter().copied() {
        empty_rows[galaxy_r] = false;
        empty_cols[galaxy_c] = false;
    }

    (galaxies, empty_rows, empty_cols)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
");
        assert_eq!(result, "8410");
    }
}