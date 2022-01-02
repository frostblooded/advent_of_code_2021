use std::cmp::{max, min};
use std::fs;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Range {
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct Board {
    data: Vec<Vec<u32>>,
}

impl Board {
    fn add_range(&mut self, range: Range) {
        if range.start.x == range.end.x {
            let y_start = min(range.start.y, range.end.y);
            let y_end = max(range.start.y, range.end.y);

            for i in y_start..=y_end {
                self.data[range.start.x as usize][i as usize] += 1;
            }
        } else if range.start.y == range.end.y {
            let x_start = min(range.start.x, range.end.x);
            let x_end = max(range.start.x, range.end.x);

            for i in x_start..=x_end {
                self.data[i as usize][range.start.y as usize] += 1;
            }
        } else {
            // Diagonal
            let left_x: u32 = min(range.start.x, range.end.x);

            let (left_point, right_point): (Point, Point) = if left_x == range.start.x {
                (range.start, range.end)
            } else {
                (range.end, range.start)
            };

            if left_point.y < right_point.y {
                // Ascending
                for (i, x) in (left_point.x..=right_point.x).enumerate() {
                    self.data[x as usize][left_point.y as usize + i] += 1;
                }
            } else {
                // Descending
                for (i, x) in (left_point.x..=right_point.x).enumerate() {
                    self.data[x as usize][left_point.y as usize - i] += 1;
                }
            }
        }
    }

    fn get_overlaps_count(&self) -> u32 {
        let mut res: u32 = 0;

        for row in &self.data {
            for &cell in row {
                if cell >= 2 {
                    res += 1;
                }
            }
        }

        res
    }

    const BOARD_SIZE: usize = 1000;

    fn new() -> Self {
        Board {
            data: vec![vec![0; Board::BOARD_SIZE]; Board::BOARD_SIZE],
        }
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Could not read input");
    let ranges = input.lines().map(|line| {
        let mut iter = line
            .split(" -> ")
            .flat_map(|s| s.split(','))
            .map(|s| s.parse::<u32>().expect("Input number parse failed"));

        Range {
            start: Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
            },
            end: Point {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
            },
        }
    });

    let mut board: Board = Board::new();

    ranges.for_each(|range| board.add_range(range));
    let res: u32 = board.get_overlaps_count();

    println!("Result: {}", res);
}
