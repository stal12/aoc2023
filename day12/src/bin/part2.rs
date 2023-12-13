use std::iter::repeat;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let input_str = include_str!("input.txt");
    print!("part 2: {}\n", part2(input_str));
}
fn part2(input: &str) -> String {
    let lines = parse_input(input);
    print!("{:?}\n", lines);
    let mut sum = 0;
    for (i, (line, correct_groups)) in lines.iter().enumerate() {
        let count = count_arrangements(line, correct_groups);
        print!("{}:\t{}\n", i, count);
        sum += count;
    }
    sum.to_string()
}

fn count_arrangements_rec(line: &[u8], correct_groups: &[i32], count: &mut i64, mut pos: usize, group: usize, mut to_leave: i32) {
    if group == correct_groups.len() {
        for i in pos..line.len() {
            if line[i] == '#' as u8 {
                return;
            }
        }
        *count += 1;
        return;
    }

    if group > 0 {
        if line[pos] != '.' as u8 && line[pos] != '?' as u8 {
            return;
        }
        pos += 1;
    }

    let group_size = correct_groups[group];
    to_leave -= group_size + 1;

    let mut max_start = line.len() - to_leave as usize - group_size as usize;
    for i in pos..max_start {
        if line[i] == '#' as u8 {
            max_start = i;
            break;
        }
    }
    for start in pos..=max_start {
        let mut found = true;
        for i in 0..group_size as usize {
            if line[start + i] != '#' as u8 && line[start + i] != '?' as u8 {
                found = false;
                break;
            }
        }
        if found {
            count_arrangements_rec(line, correct_groups, count, start + group_size as usize, group + 1, to_leave)
        }
    }
}

fn count_arrangements(line: &str, correct_groups: &[i32]) -> i64 {
    let line_tmp: Vec<u8> = line.bytes().collect();
    let mut count = 0;
    let to_leave: i32 = correct_groups.iter().sum::<i32>() + correct_groups.len() as i32;
    count_arrangements_rec(&line_tmp, correct_groups, &mut count,0, 0, to_leave);
    count
}

fn parse_line(input: &str) -> (String, Vec<i32>) {
    let str_re = Regex::new(r"[\?\.#]+").unwrap();
    let num_re = Regex::new(r"\d+").unwrap();
    let line = str_re.find(input).unwrap().as_str();
    let lengths: Vec<i32> = num_re.find_iter(input)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .collect();
    let line_times_5 = repeat(line)
        .take(5)
        .join("?");
    let length_times_5 = lengths.iter().copied().cycle().take(lengths.len() * 5).collect();
    (line_times_5, length_times_5)
}

fn parse_input(input: &str) -> Vec<(String, Vec<i32>)> {
    input.lines().map(|line| parse_line(line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
");
        assert_eq!(result, "525152");
    }
}