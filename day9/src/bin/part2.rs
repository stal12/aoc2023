use regex::Regex;

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 2: {}\n", part2(input_str));
}

fn part2(input: &str) -> String {
    let histories = parse_input(input);
    let sum: i32 = histories.iter()
        .map(|history| find_prev(history))
        .sum();
    sum.to_string()
}

fn find_prev(sequence: &[i32]) -> i32 {
    let all_differences = expand_differences(sequence);
    let mut prev_value = 0;
    for i in (0..all_differences.len()-1).rev() {
        let first_value = all_differences[i].first().unwrap().clone();
        let first_difference = prev_value;
        prev_value = first_value - first_difference;
    }
    prev_value
}

fn expand_differences(sequence: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    result.push(sequence.to_vec());
    while !result.last().unwrap().iter().all(|&x| x == 0) {
        result.push(differences(result.last().unwrap()));
    }
    result
}

fn differences(sequence: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(sequence.len() - 1);
    for i in 0..sequence.len()-1 {
        result.push(sequence[i+1] - sequence[i]);
    }
    result
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    let re = Regex::new(r"-?\d+").unwrap();
    for line in input.lines() {
        let sequence: Vec<i32> = re.find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        result.push(sequence);
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
");
        assert_eq!(result, "2");
    }
}