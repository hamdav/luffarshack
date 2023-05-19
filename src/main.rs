use std::process::Command;
use std::time::Duration;
use std::thread;
use std::{fs,io};

#[macro_use]
extern crate lazy_static;

mod hobo;

struct Player{
    cmd: Command,
    name: String,
}

fn game(p1: &mut Player, p2: &mut Player) -> io::Result<u32> {
    // Create a new game
    let mut hobogame = hobo::HoboGame::new();

    let mut current_player = 1;
    // loop alternating moves until someone wins
    loop {
        // Write game state to file
        fs::write("game.txt", hobogame.to_string())?;

        // Let the player make a move
        if current_player == 1 {
            p1.cmd.spawn()
                .expect("p1 command failed");
        } else {
            p2.cmd.spawn()
                .expect("p2 command failed");
        }

        // Wait for exactly one second
        thread::sleep(Duration::from_secs(1));

        // Read the result
        let res = fs::read_to_string("move.txt").ok()
            .and_then(|s| hobo::Coord::from_string(&s))
            .and_then(|c| hobogame.make_move(c).ok());
        // If not valid, the other player wins
        if res.is_none() {
            return Ok(3 - current_player);
        }

        // Check if this won the game
        match hobogame.winner {
            Some(hobo::Mark::X) => return Ok(1),
            Some(hobo::Mark::O) => return Ok(2),
            None => (),
        }

        // Change player, continue
        current_player = 3 - current_player;
    }
}

fn main() {
}
