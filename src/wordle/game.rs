use crate::wordle::board;
use crate::wordle::gameplay;
use crate::wordle::words;

const GUESSES_MAX: usize = 6;
const WORD_LENGTH: usize = 5;

pub struct Game {
	board: board::Board,
}

impl Game {
	pub fn new() -> Self {
		Game {
			board: board::Board::new(GUESSES_MAX, WORD_LENGTH),
		}
	}

	pub fn start(&mut self) {
		self.initialize();
		self.run();
		self.finish();
	}
	fn initialize(&self) {
		println!("--- Welcome to Wordle.rs ---");
		self.board.print();
	}

	fn run(&mut self) {
		let mut num_guesses = 0;
		let answer = words::get_random_word();

		while num_guesses < GUESSES_MAX {
			let guess = match gameplay::get_and_validate_player_guess(WORD_LENGTH) {
				Ok(v) => v,
				Err(e) => {
					println!("{}", e);
					continue;
				}
			};

			self.board.update(&guess, &answer, num_guesses);

			num_guesses += 1;
			println!("You have guessed {}/{} times", num_guesses, GUESSES_MAX);

			if guess == answer {
				println!("You guessed the correct word. You win!");
				break;
			}

			if num_guesses == GUESSES_MAX {
				println!(
					"You did not guess the word. The word was {}. You lose!",
					answer
				);
				break;
			}
		}
	}

	fn finish(&self) {
		println!("--- Game over! ---");
	}
}
