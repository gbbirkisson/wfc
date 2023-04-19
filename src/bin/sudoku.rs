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
    fn state(&self) -> CellState<u32> {
        match self {
            SudokuCell::Value(v) => CellState::Value(*v),
            SudokuCell::Superpositions(s) => CellState::Entropy(s.len()),
        }
    }

    fn constrain(&mut self, value: &u32) {
        match self {
            SudokuCell::Value(_) => return,
            SudokuCell::Superpositions(s) => {
                *s = s
                    .iter()
                    .filter(|v| v != &value)
                    .map(|v| v.clone())
                    .collect();
            }
        }
    }

    fn collapse(&mut self, value: Option<u32>) -> u32 {
        let value = match self {
            SudokuCell::Value(_) => panic!("Called collapse on a collapsed cell"),
            SudokuCell::Superpositions(s) => match value {
                Some(value) => s
                    .iter()
                    .filter(|v| *v == &value)
                    .next()
                    .expect("Value to collapse with is not in superposition")
                    .clone(),
                None => s.choose(&mut rand::thread_rng()).unwrap().clone(),
            },
        };
        *self = Self::Value(value);
        value
    }
}

#[inline]
fn get_row_index(id: usize) -> usize {
    (id as f32 / 9.0).floor() as usize
}

#[inline]
fn get_column_index(id: usize) -> usize {
    id % 9
}

#[inline]
fn get_box_index(id: usize) -> usize {
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
            neighbour_rows[get_row_index(id)].push(id);
            neighbour_columns[get_column_index(id)].push(id);
            neighbour_boxes[get_box_index(id)].push(id);
        }

        Self {
            cells: (0..81).map(|_| SudokuCell::new()).collect(),
            neighbour_rows,
            neighbour_columns,
            neighbour_boxes,
        }
    }

    fn solution(&self) -> String {
        let mut res = String::with_capacity(81);
        for c in self.cells.iter() {
            match c {
                SudokuCell::Value(v) => res.push_str(&format!("{}", v)),
                SudokuCell::Superpositions(_) => res.push('?'),
            }
        }
        res
    }
}

impl Wfc<usize, u32> for Sudoku {
    fn get_cell_with_lowest_entropy(&self) -> Option<(usize, usize)> {
        let mut cells_with_entropy: Vec<(usize, usize)> = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, c)| match c.state() {
                CellState::Entropy(_) => true,
                CellState::Value(_) => false,
            })
            .map(|(i, c)| match c.state() {
                CellState::Entropy(e) => (i, e),
                CellState::Value(_) => panic!("This should not happen"),
            })
            .collect();

        cells_with_entropy.sort_by(|(_, e1), (_, e2)| e1.cmp(e2));

        let lowest_entropy = cells_with_entropy.iter().next().map(|(_, e)| e).cloned();

        let result = if let Some(lowest_entropy) = lowest_entropy {
            cells_with_entropy = cells_with_entropy
                .into_iter()
                .filter(|(_, e)| e == &lowest_entropy)
                .collect();
            cells_with_entropy.choose(&mut rand::thread_rng())
        } else {
            None
        };
        result.copied()
    }

    fn get_cell_neighbours(&self, id: &usize) -> Vec<&usize> {
        let rows = self
            .neighbour_rows
            .get(get_row_index(*id))
            .expect("Failed to get neighbour rows")
            .iter();
        let columns = self
            .neighbour_columns
            .get(get_column_index(*id))
            .expect("Failed to get neighbour columns")
            .iter();
        let boxes = self
            .neighbour_boxes
            .get(get_box_index(*id)) // ???
            .expect("Failed to get neighbour boxes")
            .iter();
        let res: HashSet<&usize> = rows
            .chain(columns)
            .chain(boxes)
            .filter(|i| i != &id)
            .collect();
        res.into_iter().collect()
    }

    fn get_cell_state(&self, id: &usize) -> CellState<u32> {
        self.cells[*id].state()
    }

    fn cell_collapse(&mut self, id: &usize, value: Option<u32>) -> u32 {
        self.cells
            .get_mut(*id)
            .expect("Failed to get cell")
            .collapse(value)
    }

    fn cell_constrain(&mut self, id: &usize, value: &u32) {
        self.cells
            .get_mut(*id)
            .expect("Failed to get cell")
            .constrain(value)
    }
}

impl From<&str> for Sudoku {
    fn from(s: &str) -> Self {
        let mut res = Self::new();
        for (i, c) in s.chars().enumerate() {
            let c = c.to_digit(10).expect("Failed to parse char");
            if c != 0 {
                res.collapse_and_propagate(&i, Some(c));
            }
        }
        res
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
            let value = match c.state() {
                CellState::Entropy(entropy) if entropy == 0 => "X".to_string(),
                CellState::Entropy(_) => "?".to_string(),
                CellState::Value(v) => format!("{}", v),
            };
            write!(f, "{} ", value)?;
        }
        writeln!(f, "|")?;
        writeln!(f, "-------------------------")?;
        Ok(())
    }
}

fn main() {
    let mut total_processed = 1;
    let mut total_failed = 0;

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut line = line.split(",");

        let puzzle = line.next().unwrap();
        let solution = line.next().unwrap();

        if total_processed % 10000 == 0 {
            // eprintln!("On puzzle {}/{}", total_processed, total_failed);
        }

        if puzzle == "quizzes" {
            continue;
        }

        let mut sudoku = Sudoku::from(puzzle);
        sudoku.collapse_all();
        let my_solution = sudoku.solution();

        if solution.contains('?') {
            // TODO: Retry
        }
        
        if solution != my_solution {
            total_failed += 1;
            println!("{},{}", puzzle, solution);
            // eprintln!("{}", sudoku);
        }

        total_processed += 1;
    }
}
