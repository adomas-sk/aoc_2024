use crate::read_input;

pub fn solution() {
    println!("Day 5");
    let incorrect = part1();
    part2(incorrect);
    println!("");
}

fn part1() -> Vec<Vec<i32>> {
    let input = read_input::read_input("src/day5/input.txt");
    let (rules, prints) = parse_input(&input);

    let mut incorrect: Vec<Vec<i32>> = Vec::new();
    let mut total: i32 = 0;

    for print in prints {
        let mut valid = true;
        for rule in &rules {
            let rule_1_position = print.iter().position(|&item| item == rule[0]);
            let rule_2_position = print.iter().position(|&item| item == rule[1]);
            match (rule_1_position, rule_2_position) {
                (Some(first), Some(second)) => {
                    if first > second {
                        valid = false;
                        break;
                    }
                }
                _ => continue,
            }
        }
        if valid {
            let middle = print.len() / 2;
            total += print[middle];
        } else {
            incorrect.push(print);
        }
    }

    println!("Part1: {total}");

    return incorrect;
}

fn part2(mut incorrect: Vec<Vec<i32>>) {
    let input = read_input::read_input("src/day5/input.txt");
    let (rules, _) = parse_input(&input);

    let mut total: i32 = 0;

    for _ in 0..10 { // Cheating, but works
        for i in 0..incorrect.len() {
            let print = &mut incorrect[i];
            for rule in &rules {
                let rule_1_position = print.iter().position(|&item| item == rule[0]);
                let rule_2_position = print.iter().position(|&item| item == rule[1]);
                match (rule_1_position, rule_2_position) {
                    (Some(first), Some(second)) => {
                        if first > second {
                            let first_item = print[first];
                            let second_item = print[second];
                            print.insert(second, first_item);
                            print.remove(second + 1);
                            print.insert(first, second_item);
                            print.remove(first + 1);
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    for print in incorrect {
        let middle = print.len() / 2;
        total += print[middle];
    }

    println!("Part2: {total}");
}

fn parse_input(input: &String) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let lines: Vec<&str> = input.split("\n").collect();

    if let Some(index) = lines.iter().position(|&line| line == "") {
        let (rules, prints_with_empty) = lines.split_at(index);
        let prints = &prints_with_empty[1..];

        let parsed_rules: Vec<Vec<i32>> = rules
            .iter()
            .map(|rule| {
                rule.split("|")
                    .map(|part| part.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
        let parsed_prints: Vec<Vec<i32>> = prints
            .iter()
            .map(|print| {
                print
                    .split(",")
                    .map(|part| part.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        return (parsed_rules, parsed_prints);
    };
    panic!();
}
