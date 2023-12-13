use regex::Regex;
use regex::bytes;

fn main() {    
    let input_str = include_str!("input.txt");
    print!("Part 1: {}\n", part1(input_str));
}

fn part1(input: &str) -> String {
    let lines = parse_input(input);
    let mut sum = 0;
    for (i, (line, correct_groups)) in lines.iter().enumerate() {
        let count = count_arrangements(line, correct_groups);
        print!("{}:\t{}\n", i, count);
        sum += count;
    }
    sum.to_string()
}

fn find_groups(line: &[u8]) -> Vec<i32> {
    let group_re = bytes::Regex::new(r"#+").unwrap();
    group_re.find_iter(line)
        .map(|m| m.len() as i32)
        .collect()
}

fn check_groups(line: &[u8], correct_groups: &[i32]) -> bool {
    find_groups(line) == correct_groups
}

fn count_arrangements_rec(line: &mut [u8], correct_groups: &[i32], count: &mut i32, mut pos: usize) {
    while pos < line.len() && line[pos] != '?' as u8 {
        pos += 1;
    }
    if pos == line.len() {
        *count += check_groups(line, correct_groups) as i32;
        return;
    }
    line[pos] = '.' as u8;
    count_arrangements_rec(line, correct_groups, count, pos + 1);
    line[pos] = '#' as u8;
    count_arrangements_rec(line, correct_groups, count, pos + 1);
    line[pos] = '?' as u8;
}

fn count_arrangements(line: &str, correct_groups: &[i32]) -> i32 {
    let mut line_tmp: Vec<u8> = line.bytes().collect();
    let mut count = 0;
    count_arrangements_rec(&mut line_tmp, correct_groups, &mut count,0);
    count
}

fn parse_line(input: &str) -> (&str, Vec<i32>) {
    let str_re = Regex::new(r"[\?\.#]+").unwrap();
    let num_re = Regex::new(r"\d+").unwrap();
    let line = str_re.find(input).unwrap().as_str();
    let lengths = num_re.find_iter(input)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect();
    (line, lengths)
}

fn parse_input(input: &str) -> Vec<(&str, Vec<i32>)> {
    input.lines().map(|line| parse_line(line))
        .collect()
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
");
        assert_eq!(result, "21");
    }
}