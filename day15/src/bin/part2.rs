use regex::Regex;

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 2: {}\n", part2(input_str));
}

fn part2(input: &str) -> String {
    let commands = parse_input(input);
    const EMPTY_MAP: Vec<(&str, u32)> = Vec::new();
    let mut boxes = [EMPTY_MAP; 256];
    for command in commands {
        run_command(&mut boxes, command);
    }

    let mut power = 0;
    for box_number in 0..256 {
        for (lens_number, &(_label, length)) in boxes[box_number].iter().enumerate() {
            power += (box_number + 1) * (lens_number + 1) * length as usize;
        }
    }
    power.to_string()
}

fn run_command<'a>(boxes: &mut [Vec<(&'a str, u32)>; 256], command: (&'a str, Option<u32>)) {
    let box_numer = step_hash(command.0);
    let map = &mut boxes[box_numer as usize];
    if command.1.is_some() {
        let key_pos = map.iter().position(|&(key, _value)| key == command.0);
        if key_pos.is_some() {
            map[key_pos.unwrap()] = (command.0, command.1.unwrap());
        } else {
            map.push((command.0, command.1.unwrap()));
        }
    }
    else {
        map.retain(|&(key, _value)| key != command.0);
    }
}

fn step_hash(step: &str) -> u32 {
    step.bytes().fold(0, |acc, x| ((acc + x as u32) * 17) % 256)
}

fn parse_command(input: &str) -> (&str, Option<u32>) {
    let label_re = Regex::new(r"[^-=]+").unwrap();
    let length_re = Regex::new(r"\d+").unwrap();
    let command = label_re.find(input).unwrap().as_str();
    let length: Option<u32> = length_re.find(input)
        .map(|val| val.as_str().parse().unwrap());
    (command, length)
}

fn parse_input(input: &str) -> Vec<(&str, Option<u32>)> {
    let re = Regex::new(r"[^,]+").unwrap();
    let commands: Vec<_> = re.find_iter(input)
        .map(|m| m.as_str())
        .map(|step| parse_command(step))
        .collect();
    commands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, "145");
    }
}