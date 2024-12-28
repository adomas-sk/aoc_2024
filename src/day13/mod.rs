use std::usize::MAX;

use crate::read_input;

pub fn solution() {
    println!("Day 13");
    part1();
    part2();
    println!("");
}

fn part1() {
    let input = read_input::read_input("src/day13/input.txt");
    let machines = parse_input(&input);

    let mut total = 0;

    for machine in machines {
        let max_iterations =
            (machine.prize.x / machine.a_key.x).max(machine.prize.x / machine.b_key.x);
        println!("-----------");
        println!("{:?}", machine);

        let mut smallest = usize::MAX;
        for a_buttons in 0..max_iterations {
            let a_x = a_buttons * machine.a_key.x;
            let a_y = a_buttons * machine.a_key.y;
            if a_x > machine.prize.x || a_y > machine.prize.y {
                break;
            }
            if (machine.prize.x - a_x) % machine.b_key.x == 0
                && (machine.prize.y - a_y) % machine.b_key.y == 0
                && (machine.prize.x - a_x) / machine.b_key.x
                    == (machine.prize.y - a_y) / machine.b_key.y
            {
                let score = a_buttons * 3 + (machine.prize.y - a_y) / machine.b_key.y;
                if smallest > score {
                    smallest = score;
                    println!(
                        "Smallest - {smallest}, a: {a_buttons}, b: {:?}",
                        (machine.prize.y - a_y) / machine.b_key.y
                    );
                }
            }
        }
        if smallest != usize::MAX {
            total += smallest;
        }
    }

    println!("Part1: {total}");
}

fn part2() {
    // let input = read_input::read_input("src/day13/input.txt");
    // let (first_list, second_list) = parse_input(&input);

    // println!("Part2: {total}");
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Machine {
    a_key: Point,
    b_key: Point,
    prize: Point,
}

fn parse_input(input: &String) -> Vec<Machine> {
    input
        .split("\n\n")
        .into_iter()
        .map(|machine| {
            let lines: Vec<&str> = machine.split("\n").collect();

            let a_key_value: Vec<&str> = lines[0].split(": ").collect();
            let a_x_y_values: Vec<usize> = a_key_value[1]
                .split(", ")
                .into_iter()
                .map(|value| {
                    let (_, value) = value.split_at(2);
                    value.parse::<usize>().unwrap()
                })
                .collect();

            let b_key_value: Vec<&str> = lines[1].split(": ").collect();
            let b_x_y_values: Vec<usize> = b_key_value[1]
                .split(", ")
                .into_iter()
                .map(|value| {
                    let (_, value) = value.split_at(2);
                    value.parse::<usize>().unwrap()
                })
                .collect();

            let prize_key_value: Vec<&str> = lines[2].split(": ").collect();
            let prize_x_y_values: Vec<usize> = prize_key_value[1]
                .split(", ")
                .into_iter()
                .map(|value| {
                    let (_, value) = value.split_at(2);
                    value.parse::<usize>().unwrap()
                })
                .collect();

            return Machine {
                a_key: Point {
                    x: a_x_y_values[0],
                    y: a_x_y_values[1],
                },
                b_key: Point {
                    x: b_x_y_values[0],
                    y: b_x_y_values[1],
                },
                prize: Point {
                    x: prize_x_y_values[0],
                    y: prize_x_y_values[1],
                },
            };
        })
        .collect()
}
