use rand::Rng;
use std::{
	fs::File,
	io::{prelude::*, BufReader},
	path::Path,
};

pub fn read_words_from_file(filename: impl AsRef<Path>) -> Vec<String> {
	let file = File::open(filename).expect("File was not found!");
	let buffer = BufReader::new(file);
	buffer
		.lines()
		.map(|l| l.expect("Could not read line"))
		.collect()
}

pub fn get_random_word(words: &Vec<String>) -> String {
	let random_num = rand::thread_rng().gen_range(0..words.len());
	words
		.get(random_num)
		.expect("An error occurred generating a word")
		.to_string()
}
