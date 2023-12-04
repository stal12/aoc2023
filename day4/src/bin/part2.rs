use regex::Regex;

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 2: {}\n", part2(input_str));
}

fn part2(input: &str) -> String {
    let cards = parse_input(input);
    let mut copies = vec![1; cards.len()];

    for (i, (winning_nums, my_nums)) in cards.iter().enumerate() {
        let mut matches = 0;
        for my_num in my_nums {
            if winning_nums.contains(&my_num) {
                matches += 1;
            }
        }
        for j in i+1..i+matches+1 {
            copies[j] += copies[i]
        }
    }
    copies.iter().sum::<i32>().to_string()
}

fn parse_input(input: &str) -> Vec<(Vec<i32>, Vec<i32>)> {
    let mut result = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    for line in input.lines() {
        let line_payload = line.split(":").nth(1).unwrap();
        let payload_splitted : Vec<&str> = line_payload.split("|").collect();
        let winning_numbers : Vec<i32> = re.find_iter(payload_splitted[0])
            .map(|n| n.as_str().parse().unwrap())
            .collect();
        let my_numbers : Vec<i32> = re.find_iter(payload_splitted[1])
            .map(|n| n.as_str().parse().unwrap())
            .collect();
        result.push((winning_numbers, my_numbers));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
");
        assert_eq!(result, "30");
    }
}