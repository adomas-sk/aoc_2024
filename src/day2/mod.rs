use crate::read_input;

pub fn solution() {
    println!("Day 2");
    part1();
    part2();
    println!("");
}

fn part1() {
    let input = read_input::read_input("src/day2/input.txt");
    let reports = parse_input(&input);

    let mut total: i32 = 0;

    for report in reports {
        let mut last_number: i32 = 0;
        let mut increasing = false;
        let mut safe = true;
        for (index, score) in report.iter().enumerate() {
            if !safe {
                continue;
            }

            let number: i32 = score.parse().unwrap();

            let difference = (last_number - number).abs();
            if index > 0 {
                if difference > 3 || difference < 1 {
                    safe = false;
                }
                if increasing && last_number > number {
                    safe = false;
                } else if !increasing && last_number < number {
                    safe = false;
                }
            } else {
                let next_number: i32 = report[index + 1].parse().unwrap();
                if next_number > number {
                    increasing = true;
                }
            }
            last_number = number;
        }

        if safe {
            total += 1;
        }
    }

    println!("Part1: {total}");
}

fn part2() {
    let input = read_input::read_input("src/day2/input.txt");
    let reports = parse_input(&input);

    let mut total: i32 = 0;

    for report in reports {
        let mut safe = false;
        for (remove_index, _) in report.iter().enumerate() {
            let mut new_report = report.clone();
            new_report.remove(remove_index);

            let mut last_number: i32 = 0;
            let mut increasing = false;
            let mut safe_removed = true;

            for (index, score) in new_report.iter().enumerate() {
                if !safe_removed {
                    continue;
                }

                let number: i32 = score.parse().unwrap();

                let difference = (last_number - number).abs();
                if index > 0 {
                    if difference > 3 || difference < 1 {
                        safe_removed = false;
                    }
                    if increasing && last_number > number {
                        safe_removed = false;
                    } else if !increasing && last_number < number {
                        safe_removed = false;
                    }
                } else {
                    let next_number: i32 = new_report[index + 1].parse().unwrap();
                    if next_number > number {
                        increasing = true;
                    }
                }
                last_number = number;
            }
            if safe_removed {
                safe = true;
                continue;
            }
        }

        if safe {
            total += 1;
        }
    }

    println!("Part2: {total}");
}

fn parse_input(input: &String) -> Vec<Vec<&str>> {
    let reports: Vec<&str> = input.split("\n").collect();

    let parsed: Vec<Vec<&str>> = reports
        .iter()
        .map(|report| report.split(" ").collect())
        .collect();

    parsed
}
