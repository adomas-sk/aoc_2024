use crate::read_input;

pub fn solution() {
    println!("Day 10");
    part1();
    part2();
    println!("");
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Trail {
    map: Vec<Vec<usize>>,
}

impl Trail {
    fn get_point(&self, point: Point) -> Option<usize> {
        if point.y > self.map.len() - 1 || point.x > self.map[point.y].len() - 1 {
            return None;
        }
        return Some(self.map[point.y][point.x]);
    }

    fn get(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }
        let x_usize = x.abs() as usize;
        let y_usize = y.abs() as usize;
        if y_usize > self.map.len() - 1 || x_usize > self.map[y_usize].len() - 1 {
            return None;
        }
        return Some(self.map[y_usize][x_usize]);
    }

    fn find_starts(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == 0 {
                    points.push(Point { x: x, y: y });
                }
            }
        }
        return points;
    }

    fn find_higher_neighbour(&self, current: Point) -> Vec<Point> {
        let mut neighbours: Vec<Point> = Vec::new();
        if let Some(starting) = self.get_point(current) {
            let curr_x = current.x as i32;
            let curr_y = current.y as i32;
            if let Some(up) = self.get(curr_x, curr_y - 1) {
                if up == starting + 1 {
                    neighbours.push(Point {
                        x: current.x,
                        y: current.y - 1,
                    });
                }
            };
            if let Some(down) = self.get(curr_x, curr_y + 1) {
                if down == starting + 1 {
                    neighbours.push(Point {
                        x: current.x,
                        y: current.y + 1,
                    });
                }
            };
            if let Some(left) = self.get(curr_x - 1, curr_y) {
                if left == starting + 1 {
                    neighbours.push(Point {
                        x: current.x - 1,
                        y: current.y,
                    });
                }
            };
            if let Some(right) = self.get(curr_x + 1, curr_y) {
                if right == starting + 1 {
                    neighbours.push(Point {
                        x: current.x + 1,
                        y: current.y,
                    });
                }
            };
        };

        return neighbours;
    }
}

fn part1() {
    let input = read_input::read_input("src/day10/input.txt");
    let trail = Trail {
        map: parse_input(&input),
    };

    let starting_points = trail.find_starts();

    let mut total = 0;
    for start in starting_points.iter() {
        let mut finishes: Vec<String> = Vec::new();

        let pos = *start;
        let mut queue: Vec<Point> = Vec::new();
        queue.push(pos);
        while queue.len() > 0 {
            let current = queue.remove(0);
            let possible_paths = trail.find_higher_neighbour(current);
            for path in possible_paths {
                if let Some(point) = trail.get_point(path) {
                    if point == 9 {
                        let finish = format!("{:?}:{:?}", path.x, path.y);
                        if !finishes.contains(&finish) {
                            finishes.push(finish);
                        }
                    } else {
                        queue.push(path);
                    }
                }
            }
        }
        total += finishes.len();
    }

    println!("Part1: {total}");
}

fn part2() {
    let input = read_input::read_input("src/day10/input.txt");
    let trail = Trail {
        map: parse_input(&input),
    };

    let starting_points = trail.find_starts();

    let mut total = 0;
    for start in starting_points.iter() {
        let pos = *start;
        let mut queue: Vec<Point> = Vec::new();
        queue.push(pos);
        while queue.len() > 0 {
            let current = queue.remove(0);
            let possible_paths = trail.find_higher_neighbour(current);
            for path in possible_paths {
                if let Some(point) = trail.get_point(path) {
                    if point == 9 {
                        total += 1;
                    } else {
                        queue.push(path);
                    }
                }
            }
        }
    }

    println!("Part2: {total}");
}

fn parse_input(input: &String) -> Vec<Vec<usize>> {
    let lines: Vec<&str> = input.split("\n").collect();
    return lines
        .iter()
        .map(|line| {
            let values: Vec<String> = line.chars().map(|i| String::from(i)).collect();
            values.iter().map(|i| i.parse::<usize>().unwrap()).collect()
        })
        .collect();
}
