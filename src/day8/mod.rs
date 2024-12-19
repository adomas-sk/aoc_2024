use std::cmp::min;

use crate::read_input;

pub fn solution() {
    println!("Day 7");
    part1();
    part2();
    println!("");
}

#[derive(Debug)]
struct AntennaArray {
    char: char,
    positions: Vec<(usize, usize)>,
}

fn part1() {
    let input = read_input::read_input("src/day8/input.txt");
    let map = parse_input(&input);
    let mut antennas: Vec<AntennaArray> = Vec::new();

    let max_height = map.len();
    let max_width = map[0].len();

    for (y, row) in map.iter().enumerate() {
        for (x, location) in row.iter().enumerate() {
            if *location != '.' {
                if let Some(antenna) = antennas
                    .iter_mut()
                    .find(|antenna| antenna.char == *location)
                {
                    antenna.positions.push((x, y));
                } else {
                    let new_antenna_array = AntennaArray {
                        char: *location,
                        positions: vec![(x, y)],
                    };
                    antennas.push(new_antenna_array);
                }
            }
        }
    }

    let mut all_antinodes: Vec<String> = Vec::new();

    for antenna in antennas {
        let antennas_count = antenna.positions.len();

        for i in 0..antennas_count - 1 {
            for j in i + 1..antennas_count {
                let pos_1 = (
                    antenna.positions[i].0 as isize,
                    antenna.positions[i].1 as isize,
                );
                let pos_2 = (
                    antenna.positions[j].0 as isize,
                    antenna.positions[j].1 as isize,
                );

                let antinode_offset = (pos_1.0 - pos_2.0, pos_1.1 - pos_2.1);
                let antinode_1 = (pos_1.0 + antinode_offset.0, pos_1.1 + antinode_offset.1);
                let antinode_2 = (pos_2.0 - antinode_offset.0, pos_2.1 - antinode_offset.1);

                if antinode_1.0 >= 0
                    && antinode_1.0 < (max_width as isize)
                    && antinode_1.1 >= 0
                    && antinode_1.1 < (max_height as isize)
                {
                    let position_str = format!("{:?}:{:?}", antinode_1.0, antinode_1.1);
                    if !all_antinodes.contains(&position_str) {
                        all_antinodes.push(position_str);
                    }
                }
                if antinode_2.0 >= 0
                    && antinode_2.0 < (max_width as isize)
                    && antinode_2.1 >= 0
                    && antinode_2.1 < (max_height as isize)
                {
                    let position_str = format!("{:?}:{:?}", antinode_2.0, antinode_2.1);
                    if !all_antinodes.contains(&position_str) {
                        all_antinodes.push(position_str);
                    }
                }
            }
        }
    }

    let total = all_antinodes.len();
    println!("Part1: {total}");
}

fn part2() {
    let input = read_input::read_input("src/day8/input.txt");
    let map = parse_input(&input);
    let mut antennas: Vec<AntennaArray> = Vec::new();

    let max_height = map.len();
    let max_width = map[0].len();

    for (y, row) in map.iter().enumerate() {
        for (x, location) in row.iter().enumerate() {
            if *location != '.' {
                if let Some(antenna) = antennas
                    .iter_mut()
                    .find(|antenna| antenna.char == *location)
                {
                    antenna.positions.push((x, y));
                } else {
                    let new_antenna_array = AntennaArray {
                        char: *location,
                        positions: vec![(x, y)],
                    };
                    antennas.push(new_antenna_array);
                }
            }
        }
    }

    let mut all_antinodes: Vec<String> = Vec::new();

    for antenna in antennas {
        let antennas_count = antenna.positions.len();

        for antenna in antenna.positions.iter() {
            let position_str = format!("{:?}:{:?}", antenna.0, antenna.1);
            if !all_antinodes.contains(&position_str) {
                all_antinodes.push(position_str);
            }
        }

        for i in 0..antennas_count - 1 {
            for j in i + 1..antennas_count {
                let pos_1 = (
                    antenna.positions[i].0 as isize,
                    antenna.positions[i].1 as isize,
                );
                let pos_2 = (
                    antenna.positions[j].0 as isize,
                    antenna.positions[j].1 as isize,
                );

                let antinode_offset = (pos_1.0 - pos_2.0, pos_1.1 - pos_2.1);
                let max_iterations = min(
                    (max_width as isize) / antinode_offset.0.abs(),
                    (max_height as isize) / antinode_offset.1.abs(),
                );
                for i in 1..max_iterations + 1 {
                    let antinode = (
                        pos_1.0 + (antinode_offset.0 * i),
                        pos_1.1 + (antinode_offset.1 * i),
                    );
                    if antinode.0 >= 0
                        && antinode.0 < (max_width as isize)
                        && antinode.1 >= 0
                        && antinode.1 < (max_height as isize)
                    {
                        let position_str = format!("{:?}:{:?}", antinode.0, antinode.1);
                        if !all_antinodes.contains(&position_str) {
                            all_antinodes.push(position_str);
                        }
                    } else {
                        break;
                    }
                }
                for i in 1..max_iterations + 1 {
                    let antinode = (
                        pos_2.0 - (antinode_offset.0 * i),
                        pos_2.1 - (antinode_offset.1 * i),
                    );
                    if antinode.0 >= 0
                        && antinode.0 < (max_width as isize)
                        && antinode.1 >= 0
                        && antinode.1 < (max_height as isize)
                    {
                        let position_str = format!("{:?}:{:?}", antinode.0, antinode.1);
                        if !all_antinodes.contains(&position_str) {
                            all_antinodes.push(position_str);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let total = all_antinodes.len();
    println!("Part2: {total}");
}

fn parse_input(input: &String) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.split("\n").collect();
    let map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    return map;
}
