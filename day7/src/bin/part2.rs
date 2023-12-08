use std::collections::HashSet;

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 1: {}\n", part2(input_str));
}

fn part2(input: &str) -> String {
    let hands: Vec<(&str, u32)> = parse_input(input);
    print!("{:?}\n", hands);
    let mut number_hands: Vec<(u32, u32)> = hands.iter()
        .map(|(hand, bid)| (hand_to_number(hand), bid.clone()))
        .collect();
    print!("{:?}\n", number_hands);
    for (number_hand, _) in &number_hands {
        print!("{:#x}\n", number_hand);
    }

    number_hands.sort_by_key(|(number_hand, _)| *number_hand);
    let winnings: u32 = number_hands.iter().enumerate()
        .map(|(i, (_, bid))| (i as u32 +1) * *bid)
        .sum();

    winnings.to_string()
}

fn hand_to_number(hand: &str) -> u32 {
    let kind = hand_kind(hand);
    hand.chars()
        .map(|letter| letter_to_digit(letter))
        .fold(kind, |acc, digit| acc * 16 + digit)
}

fn hand_kind(hand: &str) -> u32 {
    let js = hand.chars().filter(|c| *c == 'J').count();
    let hand_no_j: String = hand.chars().filter(|c| *c != 'J').collect();
    if hand_no_j.is_empty() {
        return 6;
    }
    let char_set: HashSet<_> = hand_no_j.chars().collect();
    let char_vec: Vec<_> = char_set.iter().copied().collect();

    let highest_count = char_vec.iter()
        .map(|target| hand_no_j.chars().filter(|c| *c == *target).count())
        .max().unwrap();

    match char_set.len() {
        1 => 6,
        2 => match highest_count + js {
            4 => 5,
            3 => 4,
            _ => panic!()
        },
        3 => match highest_count + js {
            3 => 3,
            2 => 2,
            _ => panic!()
        },
        4 => 1,
        5 => 0,
        _ => panic!()
    }
}

fn letter_to_digit(letter: char) -> u32 {
    match letter {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        '2'..='9' => letter.to_digit(10).unwrap(),
        _ => panic!()
    }
}

fn parse_input(input: &str) -> Vec<(&str, u32)> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let cards = parts[0];
        let bid: u32 = parts[1].parse().unwrap();
        hands.push((cards, bid));
    }
    hands
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
");
        assert_eq!(result, "5905");
    }
}