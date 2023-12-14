use std::iter::repeat;
use regex::Regex;
use itertools::Itertools;
use std::collections::HashMap;

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

fn count_arrangements_rec(line: &[u8], correct_groups: &[i32], start_pos: usize, group: usize, mut to_leave: i32,
                          hashtag_from_here: &[i32], group_sum_from_here: i32, memo: &mut HashMap<(usize, usize), i64>) -> i64 {
    let memo_result = memo.get(&(start_pos, group));
    if memo_result.is_some() {
        return memo_result.unwrap().clone();
    }

    let mut pos = start_pos;

    if pos == line.len() {
        return 1;
    }

    if hashtag_from_here[pos] > group_sum_from_here {
        return 0;
    }

    if group == correct_groups.len() {
        return 1;
    }

    if group > 0 {
        if line[pos] != '.' as u8 && line[pos] != '?' as u8 {
            return 0;
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
    let mut nested_solutions = 0;
    for start in pos..=max_start {
        let mut found = true;
        for i in 0..group_size as usize {
            if line[start + i] != '#' as u8 && line[start + i] != '?' as u8 {
                found = false;
                break;
            }
        }
        if found {
            nested_solutions += count_arrangements_rec(line, correct_groups, start + group_size as usize,
                                                       group + 1, to_leave, hashtag_from_here, group_sum_from_here - group_size, memo);
        }
    }
    memo.insert((start_pos, group), nested_solutions);
    nested_solutions
}

fn count_arrangements(line: &str, correct_groups: &[i32]) -> i64 {
    let line_tmp: Vec<u8> = line.bytes().collect();
    let to_leave: i32 = correct_groups.iter().sum::<i32>() + correct_groups.len() as i32;
    let mut hashtag_from_here = vec![0; line_tmp.len()];
    let mut count_hashtag = 0;
    for i in (0..line_tmp.len()).rev() {
        if line_tmp[i] == '#' as u8 {
            count_hashtag += 1;
        }
        hashtag_from_here[i] = count_hashtag;
    }
    let mut memo: HashMap<(usize, usize), i64> = HashMap::new();
    let solutions = count_arrangements_rec(&line_tmp, correct_groups, 0, 0, to_leave, &hashtag_from_here, correct_groups.iter().sum(), &mut memo);
    solutions
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