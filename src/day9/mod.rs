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

fn part2() {}

fn parse_input(input: &String) -> Vec<usize> {
    let parsed: Vec<usize> = input
        .chars()
        .map(|char| String::from(char).parse::<usize>().unwrap())
        .collect();
    return parsed;
}
