use crate::read_input;

pub fn solution() {
    println!("Day 9");
    part1();
    part2();
    println!("");
}

fn parse_disk_space(compressed: &Vec<usize>) -> Vec<String> {
    let mut disk_space: Vec<String> = Vec::new();

    let mut is_file = true;
    let mut file_id = 0;
    for number in compressed {
        match is_file {
            true => {
                for _ in 0..*number {
                    disk_space.push(file_id.to_string());
                }
                file_id += 1;
            }
            false => {
                for _ in 0..*number {
                    disk_space.push(String::from("."));
                }
            }
        }
        is_file = !is_file;
    }
    return disk_space;
}

fn compact_disk_space(disk_space: &Vec<String>) -> Vec<&String> {
    let mut compact: Vec<&String> = Vec::new();

    let mut start_index: usize = 0;
    let mut end_index = disk_space.len();
    loop {
        if disk_space[start_index] != "." {
            compact.push(&disk_space[start_index]);
        } else {
            for end in (0..end_index).rev() {
                if disk_space[end] != "." {
                    end_index = end;
                    compact.push(&disk_space[end]);
                    break;
                }
            }
        }
        start_index += 1;
        if end_index - 1 < start_index {
            break;
        }
    }

    return compact;
}

fn part1() {
    let input = read_input::read_input("src/day9/input.txt");
    let list = parse_input(&input);

    let disk_space = parse_disk_space(&list);

    let compact = compact_disk_space(&disk_space);

    let mut total = 0;
    for (i, &number) in compact.iter().enumerate() {
        let sum = i * (*number).parse::<usize>().unwrap();
        total += sum;
    }

    println!("Part1: {total}");
}

#[derive(Debug, Clone, Copy)]
struct DiskSpace {
    empty: bool,
    id: usize,
    amount: usize,
}

fn parse_disk_space_with_meta(compressed: &Vec<usize>) -> Vec<DiskSpace> {
    let mut disk_spaces: Vec<DiskSpace> = Vec::new();

    let mut is_file = true;
    let mut file_id = 0;
    for number in compressed {
        let disk_space = DiskSpace {
            empty: !is_file,
            id: file_id,
            amount: *number,
        };
        disk_spaces.push(disk_space);
        if !is_file {
            file_id += 1;
        }
        is_file = !is_file;
    }
    return disk_spaces;
}

fn optimize_disk_space(disk_spaces: &Vec<DiskSpace>) -> Vec<DiskSpace> {
    let mut compact: Vec<DiskSpace> = disk_spaces.iter().map(|space| space.clone()).collect();

    loop {
        let mut found_optimization = false;

        'outer: for end_index in (0..compact.len()).rev() {
            let end = compact[end_index];
            if !end.empty {
                for start_index in 0..end_index {
                    let start = compact[start_index];
                    if start.empty && start.amount >= end.amount {
                        if start.amount == end.amount {
                            let empty_for_end = DiskSpace {
                                empty: true,
                                id: 0,
                                amount: end.amount,
                            };
                            let temp = compact
                                .splice(end_index..end_index + 1, vec![empty_for_end])
                                .collect::<Vec<DiskSpace>>();
                            compact.splice(start_index..start_index + 1, temp);
                        } else {
                            let empty_for_start = DiskSpace {
                                empty: true,
                                id: 0,
                                amount: start.amount - end.amount,
                            };
                            let empty_for_end = DiskSpace {
                                empty: true,
                                id: 0,
                                amount: end.amount,
                            };
                            let mut temp = compact
                                .splice(end_index..end_index + 1, vec![empty_for_end])
                                .collect::<Vec<DiskSpace>>();
                            temp.push(empty_for_start);
                            compact.splice(start_index..start_index + 1, temp);
                        }
                        found_optimization = true;
                        break 'outer;
                    }
                }
            }
        }
        if !found_optimization {
            break;
        }
    }

    return compact;
}

fn part2() {
    let input = read_input::read_input("src/day9/input.txt");
    let list = parse_input(&input);

    let disk_space = parse_disk_space_with_meta(&list);

    let optimized = optimize_disk_space(&disk_space);

    let mut final_disk: Vec<String> = Vec::new();
    for space in optimized.iter() {
        for _ in 0..space.amount {
            if space.empty {
                final_disk.push(String::from("."));
            } else {
                final_disk.push(space.id.to_string());
            }
        }
    }

    let mut total = 0;
    for (i, number) in final_disk.iter().enumerate() {
        if number != "." {
            let sum = i * (*number).parse::<usize>().unwrap();
            total += sum;
        }
    }

    println!("Part2: {total}");
}

fn parse_input(input: &String) -> Vec<usize> {
    let parsed: Vec<usize> = input
        .chars()
        .map(|char| String::from(char).parse::<usize>().unwrap())
        .collect();
    return parsed;
}
