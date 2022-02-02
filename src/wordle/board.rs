use colored::*;

pub struct Board {
	cells: Vec<String>,
	word_length: usize,
}

impl Board {
	pub fn new(guesses_max: usize, word_length: usize) -> Self {
		Board {
			cells: vec!["[ ]".blue().to_string(); guesses_max * word_length],
			word_length: word_length,
		}
	}

	pub fn print(&self) {
		for index in 0..self.cells.len() {
			if index % self.word_length == 0 {
				print!("\n");
			}
			print!("{}", self.cells[index]);
		}

		println!();
		println!();
	}

	pub fn update(&mut self, guess: &String, answer: &String, offset: usize) {
		let formatted_guess = self.format_guess(guess, answer);
		self.set_cell_values(formatted_guess, offset);
		self.print();
	}

	fn set_cell_values(&mut self, cell_values: Vec<String>, offset: usize) {
		for i in 0..cell_values.len() {
			let cell_index = (self.word_length * offset) + i;

			self.cells[cell_index] = cell_values
				.get(i)
				.expect("An error occurred updating the board")
				.to_string();
		}
	}

	fn format_guess(&self, guess: &String, answer: &String) -> Vec<String> {
		let mut formatted_guess = Vec::new();
		for i in 0..guess.len() {
			let guess_value = guess.chars().nth(i).unwrap();
			let answer_value = answer.chars().nth(i).unwrap();
			if guess_value == answer_value {
				formatted_guess.push(format!("[{}]", guess_value).on_green().black().to_string())
			} else if answer.contains(guess_value) {
				formatted_guess.push(format!("[{}]", guess_value).on_yellow().black().to_string())
			} else {
				formatted_guess.push(format!("[{}]", guess_value).to_string())
			}
		}
		formatted_guess
	}
}
