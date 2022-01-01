use std::fs;

#[derive(Debug, Clone)]
struct BoardCell {
    data: u8,
    marked: bool,
}

impl BoardCell {
    fn new(data: u8) -> Self {
        BoardCell {
            data,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Debug)]
struct Board {
    data: Vec<Vec<BoardCell>>,
    won: bool,
}

impl Board {
    fn push(&mut self, row: Vec<BoardCell>) {
        self.data.push(row);
    }

    fn new() -> Self {
        Board {
            data: vec![],
            won: false,
        }
    }

    fn mark_number(&mut self, number: u8) {
        for row in &mut self.data {
            for cell in row {
                if cell.data == number {
                    cell.mark();
                }
            }
        }
    }

    fn has_won(&mut self) -> bool {
        if self.won {
            return true;
        }

        // If there is a completed row
        if self
            .data
            .iter()
            .any(|row| row.iter().all(|cell| cell.marked))
        {
            self.won = true;
            return true;
        }

        let row_len = self.data[0].len();

        // Check if a col is completed
        for col_idx in 0..row_len {
            let mut col_cells: Vec<&BoardCell> = vec![];

            for row in &self.data {
                col_cells.push(&row[col_idx]);
            }

            if col_cells.iter().all(|cell| cell.marked) {
                self.won = true;
                return true;
            }
        }

        let mut diag_cells: Vec<&BoardCell> = vec![];

        for i in 0..row_len {
            diag_cells.push(&self.data[i][i]);
        }

        if diag_cells.iter().all(|cell| cell.marked) {
            self.won = true;
            return true;
        }

        diag_cells.clear();

        for i in 0..row_len {
            diag_cells.push(&self.data[row_len - i - 1][i]);
        }

        if diag_cells.iter().all(|cell| cell.marked) {
            self.won = true;
            return true;
        }

        false
    }

    fn unmarked_sum(&self) -> u32 {
        let mut res: u32 = 0;

        for row in &self.data {
            for cell in row {
                if !cell.marked {
                    res += cell.data as u32;
                }
            }
        }

        res
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Input file read failed");
    let mut lines = input.lines().peekable();
    let numbers: Vec<u8> = lines
        .next()
        .expect("No first input line found")
        .split(',')
        .map(|n| n.parse::<u8>().expect("Input number parse failed"))
        .collect();

    // Skip empty line
    lines.next().expect("Expected blank second line not found");

    let mut boards: Vec<Board> = vec![];

    while lines.peek().is_some() {
        let mut new_board: Board = Board::new();

        for i in 0..5 {
            format!("Line {}", i);
            let line = lines
                .next()
                .expect(&format!("Can't find line {} of board", i));

            new_board.push(
                line.split(' ')
                    .filter(|n| !n.is_empty())
                    .map(|n| BoardCell::new(n.parse::<u8>().expect("Couldn't parse board number")))
                    .collect(),
            );
        }

        boards.push(new_board);
        lines.next();
    }

    let mut boards_left_to_win: u32 = boards.len() as u32;

    'outer: for number in numbers {
        for board in &mut boards {
            if board.has_won() {
                continue;
            }

            board.mark_number(number);

            if board.has_won() {
                if boards_left_to_win == 1 {
                    println!("Result: {}", board.unmarked_sum() * number as u32);
                    break 'outer;
                }

                boards_left_to_win -= 1;
            }
        }
    }
}
