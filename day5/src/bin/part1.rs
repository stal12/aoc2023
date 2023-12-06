use regex::Regex;

struct RangeConversion {
    dst_start: i64,
    src_start: i64,
    len: i64
}

impl RangeConversion {
    fn has_value(&self, value: i64) -> bool {
        if value >= self.src_start && value < self.src_start + self.len {
            true
        } else {
            false
        }
    }

    fn convert(&self, value: i64) -> Option<i64> {
        if self.has_value(value) {
            Some(value - self.src_start + self.dst_start)
        } else {
            None
        }
    }
}

fn convert_value(map: &Vec<RangeConversion>, value: i64) -> i64 {
    for range_conversion in map {
        if range_conversion.has_value(value) {
            return range_conversion.convert(value).unwrap();
        }
    }
    value
}

fn convert_value_total(maps: &Vec<Vec<RangeConversion>>, value: i64) -> i64 {
    let mut new_value = value;
    for map in maps {
        new_value = convert_value(map, new_value);
    }
    new_value
}

fn main() {
    let input_str = include_str!("input.txt");
    print!("Part 1: {}\n", part1(input_str));
}

fn part1(input: &str) -> String {
    let (seeds, maps) = parse_input(input);
    let minimum_location = seeds.iter()
                                            .copied()
                                            .map(|value| convert_value_total(&maps, value))
                                            .min().unwrap();
    let mut min_start = 999999999999;
    for (i, value) in seeds.iter().copied().enumerate() {
        if i % 2 == 0 {
            let converted = convert_value_total(&maps, value);
            print!("{} {}\n", value, converted);
            if converted < min_start {
                min_start = converted;
            }

            let other_converted = convert_value_total(&maps, value + seeds[i + 1] - 1);
            print!("{} {}\n", value + seeds[i + 1] - 1, other_converted);
            if other_converted < min_start {
                min_start = other_converted;
            }
        }
    }
    print!("{}\n", min_start);
    print!("{}\n", convert_value_total(&maps, 2243422640));
    minimum_location.to_string()
}

fn parse_conversion(input: &str) -> RangeConversion {
    let re = Regex::new(r"\d+").unwrap();
    let mut matches = re.find_iter(input);
    RangeConversion {
        dst_start: matches.next().unwrap().as_str().parse().unwrap(),
        src_start: matches.next().unwrap().as_str().parse().unwrap(),
        len: matches.next().unwrap().as_str().parse().unwrap(),
    }
}

fn parse_map(input: &str) -> Vec<RangeConversion> {
    input.lines().skip(1).map(|line| parse_conversion(line)).collect()
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<RangeConversion>>) {
    let re = Regex::new(r"\d+").unwrap();
    let mut block_iter = input.split("\n\n");
    let seeds_part = block_iter.next().unwrap();
    let seeds: Vec<i64> = re.find_iter(seeds_part)
                            .map(|number_match| number_match.as_str().parse().unwrap())
                            .collect();
    let maps = block_iter.map(|block| parse_map(block)).collect();
    (seeds, maps)
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
");
        assert_eq!(result, "35");
    }
}