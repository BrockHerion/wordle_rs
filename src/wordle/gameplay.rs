use colored::*;
use std::io::stdin;

pub fn get_player_guess(word_length: usize) -> Result<String, String> {
	println!("Please enter a {} letter word: ", word_length);

	let mut player_guess = String::new();
	stdin()
		.read_line(&mut player_guess)
		.expect("An error occured while reading player input");
	validate_player_guess(player_guess.trim().to_lowercase().to_string(), word_length)
}

fn validate_player_guess(guess: String, word_length: usize) -> Result<String, String> {
	if guess.len() > word_length {
		return Err(format!(
			"\nYour guess was too long! Guesses must be {} letters long\n",
			word_length
		)
		.red()
		.to_string());
	} else if guess.len() < word_length {
		return Err(format!(
			"\nYour guess was too short! Guesses must be {} letters long\n",
			word_length
		)
		.red()
		.to_string());
	}

	Ok(guess)
}
