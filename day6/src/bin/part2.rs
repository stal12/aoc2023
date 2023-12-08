use regex::Regex;

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 2: {}\n", part2(input_str));
}

fn part2(input: &str) -> String {
    let race = parse_input(input);
    print!("{:?}\n", race);

    let time = race.0 as f64;
    let distance = race.1 as f64;

    // Solve x^2 - time * x + distance
    let discr = time*time - 4.0*distance;
    let x1 = (time - discr.sqrt()) / 2.0;
    let x2 = (time + discr.sqrt()) / 2.0;

    let first_win = x1.ceil() as u64;
    let last_win = x2.floor() as u64;

    (last_win - first_win + 1).to_string()
}

fn parse_input(input: &str) -> (u64, u64) {
    let number_re = Regex::new(r"\d+").unwrap();
    let lines: Vec<_> = input.lines().collect();
    let time_line: String = lines[0].chars().filter(|c| *c != ' ').collect();
    let distance_line: String = lines[1].chars().filter(|c| *c != ' ').collect();
    let time: u64 = number_re.find(&time_line).unwrap().as_str().parse().unwrap();
    let distance: u64 = number_re.find(&distance_line).unwrap().as_str().parse().unwrap();
    (time, distance)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Time:      7  15   30
Distance:  9  40  200
");
        assert_eq!(result, "71503");
    }
}