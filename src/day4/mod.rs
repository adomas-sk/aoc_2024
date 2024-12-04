use crate::read_input;

pub fn solution() {
    println!("Day 4");
    part1();
    part2();
    println!("");
}

const POSSIBILITIES_1: [[(i32, i32); 4]; 8] = [
    // horizontal
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    [(3, 0), (2, 0), (1, 0), (0, 0)],
    // vertical
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    [(0, 3), (0, 2), (0, 1), (0, 0)],
    // diagonal tl-br
    [(0, 0), (1, 1), (2, 2), (3, 3)],
    [(3, 3), (2, 2), (1, 1), (0, 0)],
    // diagonal tr-bl
    [(0, 0), (1, -1), (2, -2), (3, -3)],
    [(3, -3), (2, -2), (1, -1), (0, 0)],
];

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn part1() {
    let input = read_input::read_input("src/day4/input.txt");
    let words = parse_input(&input);

    let mut total: i32 = 0;

    let max_y = words.len() as i32 - 1;
    let max_x = words[0].len() as i32 - 1;
    for (y, row) in words.iter().enumerate() {
        for (x, _letter) in row.iter().enumerate() {
            for possibility in POSSIBILITIES_1 {
                let mut impossible = false;
                for (index, &position) in possibility.iter().enumerate() {
                    let x_pos = position.0 + x as i32;
                    let y_pos = position.1 + y as i32;
                    if x_pos < 0 || x_pos > max_x || y_pos < 0 || y_pos > max_y {
                        impossible = true;
                        break;
                    }
                    if words[y_pos as usize][x_pos as usize] != XMAS[index] {
                        impossible = true;
                        break;
                    }
                }
                if !impossible {
                    total += 1;
                }
            }
        }
    }

    println!("Part1: {total}");
}

const POSSIBILITIES_2: [[(i32, i32); 5]; 4] = [
    // m.s
    // .a.
    // m.s
    [(0, 0), (1, 1), (2, 2), (0, 2), (2, 0)],
    // m.m
    // .a.
    // s.s
    [(0, 0), (1, 1), (2, 2), (2, 0), (0, 2)],
    // s.s
    // .a.
    // m.m
    [(2, 2), (1, 1), (0, 0), (0, 2), (2, 0)],
    // s.m
    // .a.
    // s.m
    [(2, 2), (1, 1), (0, 0), (2, 0), (0, 2)],
];

const MASMS: [char; 5] = ['M', 'A', 'S', 'M', 'S'];

fn part2() {
    let input = read_input::read_input("src/day4/input.txt");
    let words = parse_input(&input);

    let mut total: i32 = 0;

    let max_y = words.len() as i32 - 1;
    let max_x = words[0].len() as i32 - 1;
    for (y, row) in words.iter().enumerate() {
        for (x, _letter) in row.iter().enumerate() {
            for possibility in POSSIBILITIES_2 {
                let mut impossible = false;
                for (index, &position) in possibility.iter().enumerate() {
                    let x_pos = position.0 + x as i32;
                    let y_pos = position.1 + y as i32;
                    if x_pos < 0 || x_pos > max_x || y_pos < 0 || y_pos > max_y {
                        impossible = true;
                        break;
                    }
                    if words[y_pos as usize][x_pos as usize] != MASMS[index] {
                        impossible = true;
                        break;
                    }
                }
                if !impossible {
                    total += 1;
                }
            }
        }
    }

    println!("Part2: {total}");
}

fn parse_input(input: &String) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.split("\n").collect();

    let parsed: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().into_iter().collect();
            chars
        })
        .collect();

    parsed
}
