use crate::read_input;

pub fn solution() {
    println!("Day 1");
    part1();
    part2();
    println!("");
}

fn part1() {
    let input = read_input::read_input("src/day1/input.txt");
    let (mut first_list, mut second_list) = parse_input(&input);

    first_list.sort();
    second_list.sort();

    let mut total_change: i32 = 0;

    for (index, item) in first_list.iter().enumerate() {
        let first_number: i32 = item.parse().unwrap();
        let second_number: i32 = second_list[index].parse().unwrap();
        let next_number = second_number - first_number;
        total_change += next_number.abs();
    }

    println!("Part1: {total_change}");
}

fn part2() {
    let input = read_input::read_input("src/day1/input.txt");
    let (first_list, second_list) = parse_input(&input);
    let first_numbers: Vec<i32> = first_list.iter().map(|&x| x.parse().unwrap()).collect();
    let second_numbers: Vec<i32> = second_list.iter().map(|&x| x.parse().unwrap()).collect();

    let mut total: i32 = 0;

    for number in first_numbers {
      let found_numbers: Vec<&i32> = second_numbers.iter().filter(|item| **item == number).collect();
      total += number * (found_numbers.len() as i32);
    }

    println!("Part2: {total}");
}

fn parse_input(input: &String) -> (Vec<&str>, Vec<&str>) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split("   ").collect();
        let [first, second] = parts[..] else {
            panic!("Expected exactly 2 elements");
        };
        first_list.push(first);
        second_list.push(second);
    }

    (first_list, second_list)
}
