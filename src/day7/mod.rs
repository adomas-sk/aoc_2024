use crate::read_input;

pub fn solution() {
    println!("Day 7");
    part1();
    part2();
    println!("");
}

fn part1() {
    let input = read_input::read_input("src/day7/input.txt");
    let parsed = parse_input(&input);

    let mut final_total = 0;

    for line in parsed.iter() {
        if let [total, rest @ ..] = line.as_slice() {
            let rest_len: u32 = (rest.len() - 1).try_into().unwrap();
            let total_possibilities = 2u64.pow(rest_len);

            for i in 0..total_possibilities {
                let mut possibility: Vec<u8> = Vec::new();
                for j in (0..rest_len).rev() {
                    let bit = (i >> j) & 1;
                    possibility.push(bit as u8);
                }
                let mut possibility_total = rest[0];
                for (i, sign) in possibility.iter().enumerate() {
                    if sign % 2 == 0 {
                        possibility_total *= rest[i + 1]
                    } else {
                        possibility_total += rest[i + 1]
                    }
                }
                if possibility_total == *total {
                    final_total += total;
                    break;
                }
            }
        }
    }

    println!("Part1: {final_total}");
}

fn part2() {
    let input = read_input::read_input("src/day7/input.txt");
    let parsed = parse_input(&input);

    let mut final_total = 0;

    for line in parsed.iter() {
        if let [total, rest @ ..] = line.as_slice() {
            let rest_len: u32 = (rest.len() - 1).try_into().unwrap();
            let total_possibilities = 3u64.pow(rest_len);

            for i in 0..total_possibilities {
                let mut possibility: Vec<u8> = Vec::new();
                let mut current = i;
                for _ in (0..rest_len).rev() {
                    possibility.push((current % 3) as u8);
                    current = current / 3;
                }
                let mut possibility_total = rest[0];
                for (i, sign) in possibility.iter().enumerate() {
                    if sign % 3 == 0 {
                        let combination: String =
                            possibility_total.to_string() + &rest[i + 1].to_string();
                        possibility_total = combination.parse::<u64>().unwrap();
                    } else if sign % 2 == 0 {
                        possibility_total *= rest[i + 1];
                    } else {
                        possibility_total += rest[i + 1];
                    }
                }
                if possibility_total == *total {
                    final_total += total;
                    break;
                }
            }
        }
    }

    println!("Part2: {final_total}");
}

fn parse_input(input: &String) -> Vec<Vec<u64>> {
    let lines: Vec<&str> = input.split("\n").collect();
    return lines
        .iter()
        .map(|line| {
            let mut values: Vec<String> = line.split(" ").map(|i| String::from(i)).collect();
            values[0] = values[0].replace(":", "");
            values.iter().map(|i| i.parse::<u64>().unwrap()).collect()
        })
        .collect();
}
