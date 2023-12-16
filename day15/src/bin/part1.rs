use regex::Regex;

fn main() {    
    let input_str = include_str!("input.txt");
    print!("Part 1: {}\n", part1(input_str));
}

fn part1(input: &str) -> String {
    let steps = parse_input(input);
    let sum: u32 = steps.iter().map(|&step| step_hash(step)).sum();
    sum.to_string()
}

fn step_hash(step: &[u8]) -> u32 {
    step.iter().fold(0, |acc, &x| ((acc + x as u32) * 17) % 256)
}

fn parse_input(input: &str) -> Vec<&[u8]> {
    let re = Regex::new(r"[^,]+").unwrap();
    let steps: Vec<_> = re.find_iter(input).map(|m| m.as_str().as_bytes()).collect();
    steps
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, "1320");
    }
}