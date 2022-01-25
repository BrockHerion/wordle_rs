use colored::*;
use std::io;

const GUESSES_MAX: usize = 6;
const WORD_LENGTH: usize = 5;

struct Board {
	cells: Vec<String>,
}

impl Board {
	pub fn new() -> Self {
		Board {
			cells: vec!["[ ]".blue().to_string(); GUESSES_MAX * WORD_LENGTH],
		}
	}

	pub fn print(&self) {
		for index in 0..self.cells.len() {
			if index % WORD_LENGTH == 0 {
				print!("\n");
			}
			print!("{}", self.cells[index]);
		}

		println!();
		println!();
	}

	pub fn set_word(&mut self, word: &str, offset: usize) {
		for i in 0..WORD_LENGTH {
			let cell_index = (WORD_LENGTH * offset) + i;
			let cell_value = word.chars().nth(i).unwrap();
			self.cells[cell_index] = format!("[{}]", cell_value);
		}
	}
}

fn main() {
	let word = "hello";
	let mut board = Board::new();
	let mut num_guesses = 0;

	println!("--- Welcome to Wordle.rs ---");
	board.print();

	loop {
		println!("Please enter a {} letter word: ", WORD_LENGTH);

		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("An error occurred!");

		guess = guess.trim().to_string();

		if guess.len() > WORD_LENGTH {
			println!(
				"Your guess was too long! Guesses must be {} letter long",
				WORD_LENGTH
			);
			continue;
		} else if guess.len() < WORD_LENGTH {
			println!(
				"Your guess was too short! Guesses must be {} letter long",
				WORD_LENGTH
			);
			continue;
		}

		board.set_word(&guess, num_guesses);

		board.print();

		if guess == word {
			println!("You guessed the correct word, you win!");
			break;
		}

		num_guesses += 1;

		println!("You have guessed {}/{} times", num_guesses, GUESSES_MAX);
		if num_guesses == GUESSES_MAX {
			println!("You did not guess the word. Game over!\n");
			break;
		}

		println!("Your guess was incorrect, please try again\n");
	}
}
