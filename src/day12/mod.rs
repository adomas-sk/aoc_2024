use crate::read_input;

pub fn solution() {
    println!("Day 12");
    part1();
    part2();
    println!("");
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

struct Patch {
    patch: Vec<Point>,
}

impl Patch {
    fn get(&self, x: i32, y: i32) -> Option<&Point> {
        if x < 0 || y < 0 {
            return None;
        }
        let x_usize = x.unsigned_abs() as usize;
        let y_usize = y.unsigned_abs() as usize;

        return self
            .patch
            .iter()
            .find(|&point| point.x == x_usize && point.y == y_usize);
    }
}

#[derive(Debug, Clone)]
struct Garden {
    map: Vec<Vec<char>>,
}

impl Garden {
    fn get_point(&self, point: Point) -> Option<char> {
        if point.y > self.map.len() - 1 || point.x > self.map[point.y].len() - 1 {
            return None;
        }
        return Some(self.map[point.y][point.x]);
    }

    fn get(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }
        let x_usize = x.unsigned_abs() as usize;
        let y_usize = y.unsigned_abs() as usize;
        if y_usize > self.map.len() - 1 || x_usize > self.map[y_usize].len() - 1 {
            return None;
        }
        return Some(self.map[y_usize][x_usize]);
    }

    fn find_same_neighbours(&self, current: Point) -> Vec<Point> {
        let mut neighbours: Vec<Point> = Vec::new();
        if let Some(starting) = self.get_point(current) {
            let curr_x = current.x as i32;
            let curr_y = current.y as i32;
            if let Some(up) = self.get(curr_x, curr_y - 1) {
                if up == starting {
                    neighbours.push(Point {
                        x: current.x,
                        y: current.y - 1,
                    });
                }
            };
            if let Some(down) = self.get(curr_x, curr_y + 1) {
                if down == starting {
                    neighbours.push(Point {
                        x: current.x,
                        y: current.y + 1,
                    });
                }
            };
            if let Some(left) = self.get(curr_x - 1, curr_y) {
                if left == starting {
                    neighbours.push(Point {
                        x: current.x - 1,
                        y: current.y,
                    });
                }
            };
            if let Some(right) = self.get(curr_x + 1, curr_y) {
                if right == starting {
                    neighbours.push(Point {
                        x: current.x + 1,
                        y: current.y,
                    });
                }
            };
        }
        return neighbours;
    }

    fn get_same_plants(&self, start: Point) -> Vec<Point> {
        let mut same_plants: Vec<Point> = Vec::new();
        let mut items_to_check = vec![start];
        if let Some(plant) = self.get_point(start) {
            while items_to_check.len() > 0 {
                if let Some(item) = items_to_check.pop() {
                    same_plants.push(item);
                    let neighbours = self.find_same_neighbours(item);
                    for neighbour in neighbours {
                        if !same_plants.contains(&neighbour) && !items_to_check.contains(&neighbour)
                        {
                            items_to_check.push(neighbour);
                        }
                    }
                }
            }
        }
        return same_plants;
    }

    fn iterate_over_all(&self) -> usize {
        let mut total = 0;

        let mut all: Vec<Point> = Vec::new();
        for (y, _) in self.map.iter().enumerate() {
            for (x, _) in self.map[y].iter().enumerate() {
                let current = Point { x: x, y: y };
                if !all.contains(&current) {
                    let related = self.get_same_plants(current);
                    let perimeter = self.find_perimeter(related.clone());
                    total += perimeter * related.len();
                    for i in related {
                        all.push(i)
                    }
                }
            }
        }
        return total;
    }

    fn find_perimeter(&self, patch: Vec<Point>) -> usize {
        let mut perimeter = 0;
        for point in patch {
            let neighbours = self.find_same_neighbours(point);
            perimeter += 4 - neighbours.len();
        }
        return perimeter;
    }

    fn iterate_over_all_2(&self) -> usize {
        let mut total = 0;

        let mut all: Vec<Point> = Vec::new();
        for (y, _) in self.map.iter().enumerate() {
            for (x, _) in self.map[y].iter().enumerate() {
                let current = Point { x: x, y: y };
                if !all.contains(&current) {
                    let related = self.get_same_plants(current);
                    let perimeter = self.find_perimeter_2(related.clone());
                    total += perimeter * related.len();
                    for i in related {
                        all.push(i)
                    }
                }
            }
        }
        return total;
    }

    fn find_perimeter_2(&self, patch_unstruct: Vec<Point>) -> usize {
        let mut lines = 0;
        let mut tops: Vec<Point> = Vec::new();
        let mut bots: Vec<Point> = Vec::new();
        let mut lefts: Vec<Point> = Vec::new();
        let mut rights: Vec<Point> = Vec::new();

        let patch = Patch {
            patch: patch_unstruct.clone(),
        };

        for point in patch_unstruct {
            let x_i32 = point.x as i32;
            let y_i32 = point.y as i32;
            // left
            if !lefts.contains(&point) {
                if let None = patch.get(x_i32 - 1, y_i32) {
                    lines += 1;
                    let mut up_count = 0;
                    loop {
                        up_count += 1;
                        let letf_option = patch.get(x_i32 - 1, y_i32 - up_count);
                        let up_option = patch.get(x_i32, y_i32 - up_count);

                        if letf_option.is_none() && up_option.is_some() {
                            lefts.push(*up_option.unwrap());
                        } else {
                            break;
                        }
                    }
                    let mut down_count = 0;
                    loop {
                        down_count += 1;
                        let letf_option = patch.get(x_i32 - 1, y_i32 + down_count);
                        let bot_option = patch.get(x_i32, y_i32 + down_count);

                        if letf_option.is_none() && bot_option.is_some() {
                            lefts.push(*bot_option.unwrap());
                        } else {
                            break;
                        }
                    }
                };
            }
            // right
            if !rights.contains(&point) {
                if let None = patch.get(x_i32 + 1, y_i32) {
                    lines += 1;
                    let mut up_count = 0;
                    loop {
                        up_count += 1;
                        let right_option = patch.get(x_i32 + 1, y_i32 - up_count);
                        let up_option = patch.get(x_i32, y_i32 - up_count);

                        if right_option.is_none() && up_option.is_some() {
                            rights.push(*up_option.unwrap());
                        } else {
                            break;
                        }
                    }
                    let mut down_count = 0;
                    loop {
                        down_count += 1;
                        let right_option = patch.get(x_i32 + 1, y_i32 + down_count);
                        let bot_option = patch.get(x_i32, y_i32 + down_count);

                        if right_option.is_none() && bot_option.is_some() {
                            rights.push(*bot_option.unwrap());
                        } else {
                            break;
                        }
                    }
                };
            }
            // top
            if !tops.contains(&point) {
                if let None = patch.get(x_i32, y_i32 - 1) {
                    lines += 1;
                    let mut left_count = 0;
                    loop {
                        left_count += 1;
                        let up_option = patch.get(x_i32 - left_count, y_i32 - 1);
                        let left_option = patch.get(x_i32 - left_count, y_i32);

                        if up_option.is_none() && left_option.is_some() {
                            tops.push(*left_option.unwrap());
                        } else {
                            break;
                        }
                    }
                    let mut right_count = 0;
                    loop {
                        right_count += 1;
                        let up_option = patch.get(x_i32 + right_count, y_i32 - 1);
                        let right_option = patch.get(x_i32 + right_count, y_i32);

                        if up_option.is_none() && right_option.is_some() {
                            tops.push(*right_option.unwrap());
                        } else {
                            break;
                        }
                    }
                };
            }
            // bottom
            if !bots.contains(&point) {
                if let None = patch.get(x_i32, y_i32 + 1) {
                    lines += 1;
                    let mut left_count = 0;
                    loop {
                        left_count += 1;
                        let up_option = patch.get(x_i32 - left_count, y_i32 + 1);
                        let left_option = patch.get(x_i32 - left_count, y_i32);

                        if up_option.is_none() && left_option.is_some() {
                            bots.push(*left_option.unwrap());
                        } else {
                            break;
                        }
                    }
                    let mut right_count = 0;
                    loop {
                        right_count += 1;
                        let up_option = patch.get(x_i32 + right_count, y_i32 + 1);
                        let right_option = patch.get(x_i32 + right_count, y_i32);

                        if up_option.is_none() && right_option.is_some() {
                            bots.push(*right_option.unwrap());
                        } else {
                            break;
                        }
                    }
                };
            }
        }

        return lines;
    }
}

fn part1() {
    let input = read_input::read_input("src/day12/input.txt");
    let map = parse_input(&input);

    let garden = Garden { map: map };

    let total = garden.iterate_over_all();

    println!("Part1: {total}");
}

fn part2() {
    let input = read_input::read_input("src/day12/input.txt");
    let map = parse_input(&input);

    let garden = Garden { map: map };

    let total = garden.iterate_over_all_2();

    println!("Part2: {total}");
}

fn parse_input(input: &String) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.split("\n").collect();
    return lines.iter().map(|line| line.chars().collect()).collect();
}
