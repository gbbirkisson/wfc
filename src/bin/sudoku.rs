use rand::seq::SliceRandom;
use std::{
    collections::HashSet,
    fmt::{self, Display},
};
use wfc::*;

enum SudokuCell {
    Value(u32),
    Superpositions(Vec<u32>),
}

impl SudokuCell {
    fn new() -> Self {
        Self::Superpositions((1..=9).collect())
    }
}

impl Cell<u32> for SudokuCell {
    fn constrain(&mut self, value: &u32) -> Result<(), String> {
        match self {
            SudokuCell::Value(v) if v == value => Err(format!(
                "Tried to constrain a cell with value it already has"
            )),
            SudokuCell::Value(_) => Ok(()),
            SudokuCell::Superpositions(s) => {
                *s = s
                    .iter()
                    .filter(|v| v != &value)
                    .map(|v| v.clone())
                    .collect();
                Ok(())
            }
        }
    }

    fn collapse(&mut self, value: Option<u32>) -> Result<u32, String> {
        let value = match self {
            SudokuCell::Value(_) => return Err(format!("Called collapse on a collapsed cell")),
            SudokuCell::Superpositions(s) => match value {
                Some(value) => s
                    .iter()
                    .filter(|v| *v == &value)
                    .next()
                    .ok_or_else(|| format!("Value to collapse with is not in superposition"))?
                    .clone(),
                None => s
                    .choose(&mut rand::thread_rng())
                    .ok_or("Failed to find random superposition")?
                    .clone(),
            },
        };
        *self = Self::Value(value);
        Ok(value)
    }
}

struct Sudoku {
    cells: Vec<SudokuCell>,
    neighbour_rows: Vec<Vec<usize>>,
    neighbour_columns: Vec<Vec<usize>>,
    neighbour_boxes: Vec<Vec<usize>>,
}

impl Sudoku {
    fn new() -> Self {
        let mut neighbour_rows = vec![Vec::<usize>::new(); 9];
        let mut neighbour_columns = vec![Vec::<usize>::new(); 9];
        let mut neighbour_boxes = vec![Vec::<usize>::new(); 9];
        for id in 0..81 {
            neighbour_rows[Self::get_row_index(id)].push(id);
            neighbour_columns[Self::get_column_index(id)].push(id);
            neighbour_boxes[Self::get_box_index(id)].push(id);
        }

        Self {
            cells: (0..81).map(|_| SudokuCell::new()).collect(),
            neighbour_rows,
            neighbour_columns,
            neighbour_boxes,
        }
    }


    #[inline(always)]
    fn get_row_index(id: usize) -> usize {
        (id as f32 / 9.0).floor() as usize
    }

    #[inline(always)]
    fn get_column_index(id: usize) -> usize {
        id % 9
    }

    #[inline(always)]
    fn get_box_index(id: usize) -> usize {
        // We map each box manually
        //  0,  1,  2, |  3,  4,  5, |  6,  7,  8,
        //  9, 10, 11, | 12, 13, 14, | 15, 16, 17,
        // 18, 19, 20, | 21, 22, 23, | 24, 25, 26,
        //
        // 27, 28, 29, | 30, 31, 32, | 33, 34, 35,
        // 36, 37, 38, | 39, 40, 41, | 42, 43, 44,
        // 45, 46, 47, | 48, 49, 50, | 51, 52, 53,
        //
        // 54, 55, 56, | 57, 58, 59, | 60, 61, 62,
        // 63, 64, 65, | 66, 67, 68, | 69, 70, 71,
        // 72, 73, 74, | 75, 76, 77, | 78, 79, 80,
        match id {
            0 | 1 | 2 | 9 | 10 | 11 | 18 | 19 | 20 => 0,
            3 | 4 | 5 | 12 | 13 | 14 | 21 | 22 | 23 => 1,
            6 | 7 | 8 | 15 | 16 | 17 | 24 | 25 | 26 => 2,
            27 | 28 | 29 | 36 | 37 | 38 | 45 | 46 | 47 => 3,
            30 | 31 | 32 | 39 | 40 | 41 | 48 | 49 | 50 => 4,
            33 | 34 | 35 | 42 | 43 | 44 | 51 | 52 | 53 => 5,
            54 | 55 | 56 | 63 | 64 | 65 | 72 | 73 | 74 => 6,
            57 | 58 | 59 | 66 | 67 | 68 | 75 | 76 | 77 => 7,
            60 | 61 | 62 | 69 | 70 | 71 | 78 | 79 | 80 => 8,
            _ => panic!("Index out of range"),
        }
    }

    fn solution(&self) -> String {
        let mut solution = String::with_capacity(81);
        for c in self.cells.iter() {
            match c {
                SudokuCell::Value(v) => solution.push_str(&format!("{}", v)),
                SudokuCell::Superpositions(_) => solution.push('?'),
            }
        }
        solution
    }
}

impl WaveFunctionCollapse<usize, u32> for Sudoku {
    fn cells_to_collapse(&self) -> Vec<(usize, Entropy)> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, c)| match c {
                SudokuCell::Superpositions(_) => true,
                SudokuCell::Value(_) => false,
            })
            .map(|(i, c)| match c {
                SudokuCell::Superpositions(s) => (i, s.len()),
                SudokuCell::Value(_) => panic!("This should not happen"),
            })
            .collect()
    }

    fn cell_neighbours(&self, id: &usize) -> Vec<&usize> {
        let rows = self
            .neighbour_rows
            .get(Self::get_row_index(*id))
            .expect("Failed to get neighbour rows")
            .iter();
        let columns = self
            .neighbour_columns
            .get(Self::get_column_index(*id))
            .expect("Failed to get neighbour columns")
            .iter();
        let boxes = self
            .neighbour_boxes
            .get(Self::get_box_index(*id))
            .expect("Failed to get neighbour boxes")
            .iter();
        let res: HashSet<&usize> = rows
            .chain(columns)
            .chain(boxes)
            .filter(|i| i != &id)
            .collect();
        res.into_iter().collect()
    }

    fn cell_collapse(&mut self, id: &usize, value: Option<u32>) -> Result<u32, Error> {
        self.cells
            .get_mut(*id)
            .ok_or("Failed to get cell")?
            .collapse(value)
    }

    fn cell_constrain(&mut self, id: &usize, value: &u32) -> Result<(), Error> {
        self.cells
            .get_mut(*id)
            .ok_or("Failed to get cell")?
            .constrain(value)
    }
}

impl From<&str> for Sudoku {
    fn from(s: &str) -> Self {
        let mut sudoku = Self::new();
        for (i, c) in s.chars().enumerate() {
            let c = c.to_digit(10).expect("Failed to parse char");
            if c != 0 {
                sudoku
                    .collapse_one(&i, Some(c))
                    .expect("Failed to set initial state");
            }
        }
        sudoku
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "-------------------------")?;
        for (i, c) in self.cells.iter().enumerate() {
            if i % 3 == 0 {
                if i == 0 {
                    write!(f, "| ")?;
                } else {
                    if i % 27 == 0 {
                        writeln!(f, "| ")?;
                        writeln!(f, "-------------------------")?;
                        write!(f, "| ")?;
                    } else if i % 9 == 0 {
                        writeln!(f, "| ")?;
                        write!(f, "| ")?;
                    } else {
                        write!(f, "| ")?;
                    }
                }
            }
            let value = match c {
                SudokuCell::Superpositions(s) if s.len() == 0 => format!("X"),
                SudokuCell::Superpositions(_) => format!("?"),
                SudokuCell::Value(v) => format!("{}", v),
            };
            write!(f, "{} ", value)?;
        }
        writeln!(f, "|")?;
        writeln!(f, "-------------------------")?;
        Ok(())
    }
}

fn find_solution(puzzle: &str, solution: &str) -> Option<String> {
    // Solver is somewhat based on randomness, so we will retry a 1000 times to solve the hardest
    // puzzles.
    for _ in 0..1000 {
        let mut sudoku = Sudoku::from(puzzle);

        if let Err(_) = sudoku.collapse_all() {
            // Retry
            continue;
        }

        let my_solution = sudoku.solution();
        if solution.contains('?') {
            // Retry
            continue;
        }

        if solution == my_solution {
            return Some(my_solution);
        }
    }

    None
}

fn main() {
    for line in std::io::stdin().lines() {
        let line = line.expect("Failed to read line from stdin");
        // Ignoring column names in the csv
        if line == "quizzes,solutions" {
            continue;
        }

        let mut line = line.split(",");
        let sudoku = line.next().expect("Failed to get puzzle from csv");
        let solution = line.next().expect("Failed to get solution from csv");

        if find_solution(&sudoku, &solution).is_none() {
            println!("{},{}", sudoku, solution);
        }
    }
}
