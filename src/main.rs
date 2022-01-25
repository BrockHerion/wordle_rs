use colored::*;
use std::io;

const GUESSES_MAX: usize = 6;
const WORD_LENGTH: usize = 5;

struct Board {
	cells: Vec<String>,
	answer: String,
}

impl Board {
	pub fn new(answer: String) -> Self {
		Board {
			cells: vec!["[ ]".blue().to_string(); GUESSES_MAX * WORD_LENGTH],
			answer: answer,
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

			self.cells[cell_index] = self.get_cell_value(cell_value, i);
		}
	}

	fn get_cell_value(&self, guessed_char: char, index: usize) -> String {
		let answer_char = self.answer.chars().nth(index).unwrap();

		if answer_char == guessed_char {
			return format!("[{}]", guessed_char).green().to_string();
		} else if self.answer.contains(guessed_char) {
			format!("[{}]", guessed_char).yellow().to_string()
		} else {
			format!("[{}]", guessed_char).to_string()
		}
	}
}

fn main() {
	let word = "hello";
	let mut board = Board::new(String::from(word));
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

		num_guesses += 1;

		if guess == word {
			println!(
				"You guessed the correct word in {} turns, you win!",
				num_guesses
			);
			break;
		}

		println!("You have guessed {}/{} times", num_guesses, GUESSES_MAX);
		if num_guesses == GUESSES_MAX {
			println!("You did not guess the word. Your word was {}", word.red());
			break;
		}

		println!("Your guess was incorrect, please try again\n");
	}

	println!("Game over!")
}
