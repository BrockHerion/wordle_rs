mod wordle;

fn main() {
	let mut game = wordle::game::Game::new();
	game.start();
}
