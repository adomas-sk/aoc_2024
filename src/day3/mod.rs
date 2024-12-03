use std::cmp::min;

use crate::read_input;

pub fn solution() {
    println!("Day 3");
    part1();
    part2();
    println!("");
}

fn part1() {
    let input = read_input::read_input("src/day3/input.txt");

    let mut total: i32 = 0;
    let chars: Vec<char> = input.chars().into_iter().collect();

    let mut index_to_skip_to = 0;
    let total_length = chars.len();
    for (index, &c) in chars.iter().enumerate() {
        if index < index_to_skip_to {
            continue;
        }
        if total_length > index + 4
            && c == 'm'
            && chars[index + 1] == 'u'
            && chars[index + 2] == 'l'
            && chars[index + 3] == '('
        {
            let mut found_end = false;
            let mut found_comma = false;
            let mut invalid = false;
            let mut last_index = 0;
            let mut comma_index = 0;
            for i in 0..min(8, total_length - (index + 4)) {
                if chars[index + 4 + i] == ')' {
                    found_end = true;
                    last_index = index + 4 + i;
                    break;
                }
                if chars[index + 4 + i] == ',' {
                    if found_comma {
                        invalid = true;
                        break;
                    }
                    found_comma = true;
                    comma_index = index + 4 + i;
                    continue;
                }
                if !chars[index + 4 + i].is_numeric() {
                    invalid = true;
                    break;
                }
            }
            if found_end && !invalid && found_comma {
                index_to_skip_to = last_index + 1;

                let first_number_chars = &chars[index + 4..comma_index];
                let second_number_chars = &chars[comma_index + 1..last_index];

                let first_number_string: String = first_number_chars.iter().collect();
                let second_number_string: String = second_number_chars.iter().collect();

                let first_number: i32 = first_number_string.parse().unwrap();
                let second_number: i32 = second_number_string.parse().unwrap();

                total += first_number * second_number
            }
        }
    }

    println!("Part1: {total}");
}

fn part2() {
    let input = read_input::read_input("src/day3/input.txt");

    let mut total: i32 = 0;
    let chars: Vec<char> = input.chars().into_iter().collect();

    let do_command = "do()";
    let dont_command = "don't()";

    let mut index_to_skip_to = 0;
    let mut enabled = true;
    let total_length = chars.len();
    for (index, &c) in chars.iter().enumerate() {
        if index < index_to_skip_to {
            continue;
        }
        if total_length > index + 4
            && c == 'm'
            && chars[index + 1] == 'u'
            && chars[index + 2] == 'l'
            && chars[index + 3] == '('
        {
            let mut found_end = false;
            let mut found_comma = false;
            let mut invalid = false;
            let mut last_index = 0;
            let mut comma_index = 0;
            for i in 0..min(8, total_length - (index + 4)) {
                if chars[index + 4 + i] == ')' {
                    found_end = true;
                    last_index = index + 4 + i;
                    break;
                }
                if chars[index + 4 + i] == ',' {
                    if found_comma {
                        invalid = true;
                        break;
                    }
                    found_comma = true;
                    comma_index = index + 4 + i;
                    continue;
                }
                if !chars[index + 4 + i].is_numeric() {
                    invalid = true;
                    break;
                }
            }
            if found_end && !invalid && found_comma {
                index_to_skip_to = last_index + 1;

                if enabled {
                  let first_number_chars = &chars[index + 4..comma_index];
                  let second_number_chars = &chars[comma_index + 1..last_index];
  
                  let first_number_string: String = first_number_chars.iter().collect();
                  let second_number_string: String = second_number_chars.iter().collect();
  
                  let first_number: i32 = first_number_string.parse().unwrap();
                  let second_number: i32 = second_number_string.parse().unwrap();
  
                  total += first_number * second_number
                }
                continue;
            }
        }
        let dont_chars = &chars[index..min(index+7, total_length)];
        let dont_string: String = dont_chars.iter().collect();

        if dont_string == dont_command {
          index_to_skip_to = index+7;
          enabled = false;
        }

        let do_chars = &chars[index..min(index+4, total_length)];
        let do_string: String = do_chars.iter().collect();

        if do_string == do_command {
          index_to_skip_to = index+4;
          enabled = true;
        }

    }

    println!("Part2: {total}");
}
