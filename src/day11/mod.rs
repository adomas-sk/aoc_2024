use crate::read_input;
use std::collections::HashMap;

pub fn solution() {
    println!("Day 11");
    part1();
    part2();
    println!("");
}

fn get_number_of_digits(mut numb: u64) -> u32 {
    let mut digits = 0;
    while numb > 0 {
        digits += 1;
        numb = numb / 10;
    }

    return digits;
}

fn part1() {
    let input = read_input::read_input("src/day11/input.txt");
    let mut stones = parse_input(&input);

    for _ in 0..25 {
        let mut i = 0;
        while i < stones.len() {
            if stones[i] == 0 {
                stones[i] = 1;
                i += 1;
                continue;
            }
            let digits = get_number_of_digits(stones[i]);
            if digits % 2 == 0 {
                let denominator = 10u64.pow(digits / 2);
                let first_half = stones[i] / denominator;
                let second_half = stones[i] % denominator;
                stones.splice(i..i + 1, vec![first_half, second_half]);
                i += 2;
                continue;
            }
            stones[i] *= 2024;
            i += 1;
        }
    }

    let total = stones.len();

    println!("Part1: {total}");
}

fn compute(mut stones: HashMap<u64, u64>) -> u64 {
    for _ in 0..75 {
        let mut new_h: HashMap<u64, u64> = HashMap::new();
        for &stone in stones.keys() {
            if let Some(&amount) = stones.get(&stone) {
                if stone == 0 {
                    if let Some(existing_amount) = new_h.get(&1) {
                        new_h.insert(1, existing_amount + amount);
                    } else {
                        new_h.insert(1, amount);
                    }
                    continue;
                }
                let digits = get_number_of_digits(stone);
                if digits % 2 == 0 {
                    let denominator = 10u64.pow(digits / 2);
                    let first_half = stone / denominator;
                    let second_half = stone % denominator;
                    if let Some(existing_amount) = new_h.get(&first_half) {
                        new_h.insert(first_half, existing_amount + amount);
                    } else {
                        new_h.insert(first_half, amount);
                    }
                    if let Some(existing_amount) = new_h.get(&second_half) {
                        new_h.insert(second_half, existing_amount + amount);
                    } else {
                        new_h.insert(second_half, amount);
                    }
                    continue;
                }
                let new_item = stone * 2024;
                if let Some(existing_amount) = new_h.get(&new_item) {
                    new_h.insert(new_item, existing_amount + amount);
                } else {
                    new_h.insert(new_item, amount);
                }
            }
        }
        stones = new_h;
    }
    let mut total = 0;
    for (_, amount) in stones {
        total += amount;
    }
    return total;
}

fn part2() {
    let input = read_input::read_input("src/day11/input.txt");
    let init_stones = parse_input(&input);

    let mut h: HashMap<u64, u64> = HashMap::new();

    for i in 0..init_stones.len() {
        h.insert(init_stones[i], 1);
    }

    let total = compute(h);

    println!("Part2: {total}");
}

fn parse_input(input: &String) -> Vec<u64> {
    return input
        .split(" ")
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect();
}
