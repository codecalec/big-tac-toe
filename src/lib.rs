use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Marking {
    Nought,
    Cross,
}

impl fmt::Display for Marking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Marking::Nought => write!(f, "O"),
            Marking::Cross => write!(f, "X"),
        }
    }
}

#[derive(Debug)]
pub enum PlacementError {
    FilledSpaceError(usize, usize),
    InvalidLocationError(usize, usize),
}

impl fmt::Display for PlacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlacementError::FilledSpaceError(i, j) => {
                write!(f, "Space is already filled at ({},{})", i + 1, j + 1)
            }
            PlacementError::InvalidLocationError(i, j) => {
                write!(f, "Location isn't valid at ({}, {})", i + 1, j + 1)
            }
        }
    }
}

#[derive(Clone)]
pub struct InnerBoard {
    board: Vec<Vec<Option<Marking>>>,
    pub winner: Option<Marking>,
}

impl InnerBoard {
    fn new() -> Self {
        let board = vec![vec![None; 9]; 9];
        let winner = None;
        InnerBoard { board, winner }
    }

    fn contains(&self, col: usize, row: usize, m: &Marking) -> bool {
        match &self.board[col][row] {
            Some(value) => *m == *value,
            None => false,
        }
    }

    fn place(&mut self, row: &usize, col: &usize, m: &Marking) -> Result<(), PlacementError> {
        self.place_marker(row, col, m)?;

        if self.check_winner(m) {
            self.winner = Some(m.clone());
        }
        Ok(())
    }

    fn place_marker(
        &mut self,
        row: &usize,
        col: &usize,
        marking: &Marking,
    ) -> Result<(), PlacementError> {
        if *row > 3 || *col > 3 {
            return Err(PlacementError::InvalidLocationError(*row, *col));
        }
        match self.board[*row][*col] {
            None => {
                self.board[*row][*col] = Some(marking.clone());
                Ok(())
            }
            _ => Err(PlacementError::FilledSpaceError(*row, *col)),
        }
    }

    fn check_winner(&self, m: &Marking) -> bool {
        // Winning including middle
        if self.contains(1, 1, m) {
            if self.contains(1, 0, m) {
                if self.contains(1, 2, m) {
                    return true;
                }
            }

            if self.contains(0, 0, m) {
                if self.contains(2, 2, m) {
                    return true;
                }
            }

            if self.contains(0, 1, m) {
                if self.contains(2, 1, m) {
                    return true;
                }
            }

            if self.contains(0, 2, m) {
                if self.contains(2, 0, m) {
                    return true;
                }
            }
        }

        // Winning including top left block
        if self.contains(0, 0, m) {
            if self.contains(1, 0, m) {
                if self.contains(2, 0, m) {
                    return true;
                }
            }

            if self.contains(0, 1, m) {
                if self.contains(0, 2, m) {
                    return true;
                }
            }
        }

        // Winning including bottom right block
        if self.contains(2, 2, m) {
            if self.contains(2, 1, m) {
                if self.contains(2, 0, m) {
                    return true;
                }
            }

            if self.contains(1, 2, m) {
                if self.contains(0, 2, m) {
                    return true;
                }
            }
        }
        false
    }
}

impl fmt::Display for InnerBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid = String::new();

        for _ in 0..7 {
            grid.push('-');
        }
        grid.push('\n');

        for i in 0..3 {
            grid.push('|');
            for j in 0..3 {
                if let Some(m) = &self.board[i][j] {
                    grid.push_str(format!("{}", m).as_str());
                } else {
                    grid.push(' ');
                }
                grid.push('|');
            }
            grid.push('\n');

            for _ in 0..7 {
                grid.push('-');
            }
            grid.push('\n');
        }

        write!(f, "{}", grid)
    }
}

pub struct OuterBoard {
    boards: Vec<Vec<InnerBoard>>,
    pub master_board: InnerBoard,
}

impl OuterBoard {
    pub fn new() -> Self {
        let mut boards = Vec::with_capacity(3);
        for _ in 0..3 {
            let mut row = Vec::with_capacity(3);
            for _ in 0..3 {
                row.push(InnerBoard::new());
            }
            boards.push(row);
        }

        let master_board = InnerBoard::new();

        OuterBoard {
            boards,
            master_board,
        }
    }

    pub fn place(&mut self, i: usize, j: usize, row: &usize, col: &usize, m: &Marking) -> Result<(usize, usize), PlacementError> {
        self.boards[i][j].place(row, col, m)?;
        self.update_master_board(&i, &j);
        if self.master_board.check_winner(m) {
            println!("Winner is {}\n\n{}", m, self);
            std::process::exit(0);
        }
        Ok((*row, *col))
    }

    fn update_master_board(&mut self, row: &usize, col: &usize) {
        if self.master_board.board[*row][*col] == None {
            self.master_board.board[*row][*col] = self.boards[*row][*col].winner.clone();
        }
    }
}

impl fmt::Display for OuterBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = String::new();

        for _ in 0..(3 * 7 + 4) {
            grid.push('=');
        }
        grid.push('\n');

        for row in 0..3 {
            let left = &self.boards[row][0].board;
            let mid = &self.boards[row][1].board;
            let right = &self.boards[row][2].board;

            for inner_row in 0..3 {
                grid.push('‖');
                for board in [left, mid, right] {
                    grid.push('|');
                    for inner_col in 0..3 {
                        if let Some(m) = &board[inner_row][inner_col] {
                            grid.push_str(format!("{}", m).as_str());
                        } else {
                            grid.push(' ');
                        }
                        grid.push('|');
                    }

                    grid.push('‖');
                }
                grid.push('\n');

                grid.push_str("‖-------‖-------‖-------‖\n");
            }

            for _ in 0..(3 * 7 + 4) {
                grid.push('=');
            }
            grid.push('\n');
        }

        write!(f, "{}", grid)
    }
}
