use array2d::Array2D;

#[derive(Debug)]
pub struct Nonogram {
	pub size: usize,
	pub board: Array2D<bool>,
	pub col_rules: Vec<Vec<usize>>,
	pub row_rules: Vec<Vec<usize>>,
}

impl Nonogram {

	pub fn set(&mut self, row : usize, col: usize, val : bool) {
		self.board[(row, col)] = val;
	}

	pub fn get(&self, row : usize, col: usize) -> bool{
		self.board[(row, col)]
	}

	pub fn get_size(&self) -> usize {
		self.size
	}

	pub fn print(&self) {
		println!("Printing!");
		let mut leading_spaces = 0;
		for row_rule in self.row_rules.iter() {
			if row_rule.len() > leading_spaces {
				leading_spaces = row_rule.len();
			}
		}

		let mut leading_lines = 0;
		for col_rule in self.col_rules.iter() {
			if col_rule.len() > leading_lines {
				leading_lines = col_rule.len();
			}
		}

		for line in 0..leading_lines {
			for _space in 0..(leading_spaces * 2) {
				print!(" ");
			}
			for col in self.col_rules.iter() {
				if leading_lines - line <= col.len() {
					print!("{}", col[col.len() - (leading_lines - line)]);
				} else {
					print!(" ");
				}
				print!(" ");
			}
			println!();
		}

		for row in 0..self.size {
			for _space in 0..(2 * (leading_spaces - (self.row_rules[row].len()))) {
				print!(" ");
			}
			for i in 0..self.row_rules[row].len() {
				print!("{} ", self.row_rules[row][i]);
			}
			for col in 0..self.size {
				let cell = self.board[(row, col)];
				if cell {
					print!("x ");
				} else {
					print!("  ");
				}
			}
			println!();
		}
		println!();
	}

	fn check_first_set(values: &[bool], set_size : usize) -> i32 {
		let mut i = 0usize;
		while i < values.len() && !values[i] {
			i += 1;
		}

		let mut counter = 0usize;
		while i < values.len() && values[i] {
			i += 1;
			counter += 1;
		}

		if set_size != counter {
			-1
		} else {
			(i).try_into().unwrap()
		}
	}

	fn check(values: &[bool], rules: &[usize]) -> bool {
		let mut values = values;
		// Check all sets in a row
		for &rule in rules.iter() {
			let index = Nonogram::check_first_set(values, rule);
			if index == -1 {
				// Set does not exist
				return false
			}
			let slice_index : usize = index.try_into().unwrap();
			values = &values[slice_index..];
		}

		for &val in values.iter() {
			// Check that there are no remaining trues after the last set
			if val {
				return false
			}
		}
		true
	}

	pub fn validate(&self) -> bool {
		let rows = self.board.as_rows();
		let cols = self.board.as_columns();
		for i in 0..self.size {
			if !Nonogram::check(&rows[i], &self.row_rules[i]) {
				println!("Row {} invalid", i);
				return false
			}
			if !Nonogram::check(&cols[i], &self.col_rules[i]) {
				println!("Col {} invalid", i);
				return false
			}
		}
		true
	}
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn init_nonogram_from_file (filename: &str) -> Option<Nonogram> {
	println!("Reading file: {}", filename);

	let mut lines = read_lines(filename).unwrap();

	// this is horrible
	let size = lines.next().unwrap().unwrap().trim().parse().expect("File in wrong format");
	println!("{}", size);

	let mut col_rules = Vec::new();
	let mut row_rules = Vec::new();

	for _col in 0..size {
		let line = lines.next().unwrap().unwrap();
		let line = line.split(' ').map(|x| x.parse().unwrap()).collect();

		println!("{:?}", line);
		col_rules.push(line);
	}

	for _row in 0..size {
		let line = lines.next().unwrap().unwrap();
		let line = line.split(' ').map(|x| x.parse().unwrap()).collect();
		
		println!("{:?}", line);
		row_rules.push(line);
	}

	println!("cols: {:?}", col_rules);
	println!("rows: {:?}", row_rules);

	let puzzle = Nonogram{
		size,
		board: Array2D::filled_with(false, size, size),
		col_rules,
		row_rules
	};

	Some(puzzle)	
}


