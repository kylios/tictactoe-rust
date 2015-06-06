// Tic-tac-toe in Rust!

// Author: Kyle Racette <kracette at gmail dot com>

use std::result::Result;

/*
Defines a player type, which interacts with the game.
*/
struct Player {
	symbol: char,
	find_move: fn(player: &Player, board: &Game) -> (u32, u32)
}

impl PartialEq for Player {
	fn eq(&self, other: &Self) -> bool {
		self.symbol == other.symbol
	}
	fn ne(&self, other: &Self) -> bool {
		self.symbol != other.symbol
	}
}

/*
Errors which may be raised by the game
*/
enum GameError {
	InvalidStateError,
	InvalidPlacementError
}

/*
Holds game state and defines mutations to that state.
*/
struct Game<'a> {
	cols: u32,
	rows: u32,
	grid: Vec<Option<&'a Player>>,
	active_player: Option<&'a Player>,
	p1: Player,
	p2: Player
}

impl<'a> Game<'a> {
	fn new(cols: u32, rows: u32, p1: Player, p2: Player) -> Game<'a> {
		let size = cols * rows;
		let grid = vec![None; size as usize];
		Game {
			cols: cols,
			rows: rows,
			grid: grid,
			active_player: None,
			p1: p1,
			p2: p2
		}
	}

	fn start(&'a mut self) -> &'a mut Self {
		self.active_player = Some(&self.p1);
		self
	}

	fn take_move(&'a mut self, row: u32, col: u32) -> Result<(), GameError> {
		match self.active_player {
			Some(active_player) => {
				let pos = (row * self.cols + col) as usize;
				match self.grid[pos] {
					None => {
						self.grid[pos] = self.active_player;
						self.active_player = Some(if *active_player == self.p1 {
							&self.p2
						} else if *active_player == self.p2 {
							&self.p1
						} else {
							unreachable!("The active player was neither p1 nor p2")
						});
						Ok(())
					},
					_ => Err(GameError::InvalidPlacementError)
				}
			},
			None => Err(GameError::InvalidStateError)
		}
	}

	fn run(&mut self) -> Result<bool, GameError> {
		println!("running");
		Ok(true)
	}
}

fn human_find_move(player: &Player, board: &Game) -> (u32, u32) {
	(0, 0)
}

fn cpu_find_move(player: &Player, board: &Game) -> (u32, u32) {
	(0, 0)		
}



fn main() {
	
	let p1 = Player {
		symbol: 'X',
		find_move: human_find_move
	};
	let p2 = Player {
		symbol: 'O',
		find_move: cpu_find_move
	};

	let &mut board = Game::new(3, 3, p1, p2).start();
	board.start();
	loop {
		let cont = board.run().ok().unwrap();
		match cont {
			false => break,
			_ => ()
		};
	};
}