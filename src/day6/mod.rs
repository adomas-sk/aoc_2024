use std::ptr;

use crate::read_input;

pub fn solution() {
    println!("Day 6");
    part1();
    part2();
    println!("");
}

fn add_to_vec_if_missing(vector: &mut Vec<String>, item: String) {
    if vector.iter().any(|e| *e == item) {
        return;
    }
    vector.push(item);
}

fn part1() {
    let input = read_input::read_input("src/day6/input.txt");
    let matrix = parse_input(&input);

    let mut pos: (usize, usize) = (0, 0);

    for (y, row) in matrix.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if matrix[y][x] == '^' {
                pos = (x, y);
            }
        }
    }

    let mut all_places: Vec<String> = Vec::new();
    add_to_vec_if_missing(&mut all_places, format!("{}:{}", pos.0, pos.1));
    let mut inside = true;
    let mut direction = '^';
    while inside {
        if direction == '^' {
            if pos.1 == 0 {
                inside = false;
                continue;
            }
            match matrix.get(pos.1 - 1) {
                None => {
                    inside = false;
                    continue;
                }
                Some(_) => {
                    let next_cell = matrix[pos.1 - 1][pos.0];
                    if next_cell == '#' {
                        direction = '>';
                        continue;
                    }
                    add_to_vec_if_missing(&mut all_places, format!("{}:{}", pos.0, pos.1 - 1));
                    pos = (pos.0, pos.1 - 1);
                    continue;
                }
            }
        }
        if direction == '>' {
            match matrix[pos.1].get(pos.0 + 1) {
                None => {
                    inside = false;
                    continue;
                }
                Some(_) => {
                    let next_cell = matrix[pos.1][pos.0 + 1];
                    if next_cell == '#' {
                        direction = 'v';
                        continue;
                    }
                    add_to_vec_if_missing(&mut all_places, format!("{}:{}", pos.0 + 1, pos.1));
                    pos = (pos.0 + 1, pos.1);
                    continue;
                }
            }
        }
        if direction == 'v' {
            match matrix.get(pos.1 + 1) {
                None => {
                    inside = false;
                    continue;
                }
                Some(_) => {
                    let next_cell = matrix[pos.1 + 1][pos.0];
                    if next_cell == '#' {
                        direction = '<';
                        continue;
                    }
                    add_to_vec_if_missing(&mut all_places, format!("{}:{}", pos.0, pos.1 + 1));
                    pos = (pos.0, pos.1 + 1);
                    continue;
                }
            }
        }
        if direction == '<' {
            if pos.0 == 0 {
                inside = false;
                continue;
            }
            match matrix[pos.1].get(pos.0 - 1) {
                None => {
                    inside = false;
                    continue;
                }
                Some(_) => {
                    let next_cell = matrix[pos.1][pos.0 - 1];
                    if next_cell == '#' {
                        direction = '^';
                        continue;
                    }
                    add_to_vec_if_missing(&mut all_places, format!("{}:{}", pos.0 - 1, pos.1));
                    pos = (pos.0 - 1, pos.1);
                    continue;
                }
            }
        }
    }

    let total = all_places.len();

    println!("Part1: {total}");
}

#[derive(Debug, PartialEq)]
struct HistoryItem {
    position: (usize, usize),
    direction: char,
}

fn part2() {
    let input = read_input::read_input("src/day6/input.txt");
    let matrix = parse_input(&input);

    let mut init_pos: (usize, usize) = (0, 0);

    for (y, row) in matrix.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if matrix[y][x] == '^' {
                init_pos = (x, y);
            }
        }
    }

    let mut total = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, current_char) in row.iter().enumerate() {
            if *current_char != '.' {
                continue;
            }

            let mut pos: (usize, usize) = init_pos.clone();
            let mut history: Vec<HistoryItem> = Vec::new();
            let history_item = HistoryItem {
                position: (pos.0, pos.1),
                direction: '^',
            };
            history.push(history_item);
            let mut inside = true;
            let mut direction = '^';
            while inside {
                if direction == '^' {
                    if pos.1 == 0 {
                        inside = false;
                        continue;
                    }
                    match matrix.get(pos.1 - 1) {
                        None => {
                            inside = false;
                            continue;
                        }
                        Some(_) => {
                            let next_cell = &matrix[pos.1 - 1][pos.0];

                            if ptr::eq(current_char, next_cell) || *next_cell == '#' {
                                direction = '>';
                                continue;
                            }
                            pos = (pos.0, pos.1 - 1);
                            let history_item = HistoryItem {
                                position: (pos.0, pos.1),
                                direction: direction,
                            };
                            if history.contains(&history_item) {
                                total += 1;
                                break;
                            }
                            history.push(history_item);
                            continue;
                        }
                    }
                }
                if direction == '>' {
                    match matrix[pos.1].get(pos.0 + 1) {
                        None => {
                            inside = false;
                            continue;
                        }
                        Some(_) => {
                            let next_cell = &matrix[pos.1][pos.0 + 1];

                            if ptr::eq(current_char, next_cell) || *next_cell == '#' {
                                direction = 'v';
                                continue;
                            }
                            pos = (pos.0 + 1, pos.1);
                            let history_item = HistoryItem {
                                position: (pos.0, pos.1),
                                direction: direction,
                            };
                            if history.contains(&history_item) {
                                total += 1;
                                break;
                            }
                            history.push(history_item);
                            continue;
                        }
                    }
                }
                if direction == 'v' {
                    match matrix.get(pos.1 + 1) {
                        None => {
                            inside = false;
                            continue;
                        }
                        Some(_) => {
                            let next_cell = &matrix[pos.1 + 1][pos.0];

                            if ptr::eq(current_char, next_cell) || *next_cell == '#' {
                                direction = '<';
                                continue;
                            }
                            pos = (pos.0, pos.1 + 1);
                            let history_item = HistoryItem {
                                position: (pos.0, pos.1),
                                direction: direction,
                            };
                            if history.contains(&history_item) {
                                total += 1;
                                break;
                            }
                            history.push(history_item);
                            continue;
                        }
                    }
                }
                if direction == '<' {
                    if pos.0 == 0 {
                        inside = false;
                        continue;
                    }
                    match matrix[pos.1].get(pos.0 - 1) {
                        None => {
                            inside = false;
                            continue;
                        }
                        Some(_) => {
                            let next_cell = &matrix[pos.1][pos.0 - 1];

                            if ptr::eq(current_char, next_cell) || *next_cell == '#' {
                                direction = '^';
                                continue;
                            }
                            pos = (pos.0 - 1, pos.1);
                            let history_item = HistoryItem {
                                position: (pos.0, pos.1),
                                direction: direction,
                            };
                            if history.contains(&history_item) {
                                total += 1;
                                break;
                            }
                            history.push(history_item);
                            continue;
                        }
                    }
                }
            }
        }
    }

    println!("Part2: {total}");
}

fn parse_input(input: &String) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.split("\n").collect();
    return lines
        .iter()
        .map(|line| line.chars().into_iter().collect())
        .collect();
}
