use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Coord(u32, u32);

impl Coord {
    pub fn from_string(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\((\d+),(\d+)\)").unwrap();
        }
        let caps = RE.captures(s)?;
        Some(Self{
            0:caps.get(1)?.as_str().parse().ok()?,
            1:caps.get(2)?.as_str().parse().ok()?
        })

    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mark{
    X,
    O,
}

pub struct HoboGame {
    marks: HashMap<Coord, Mark>,
    turn: Mark,
    pub winner: Option<Mark>,
}

impl HoboGame {
    fn move_is_legal(&self, c: Coord) -> bool {
        /*
         * Return true if move is legal, false otherwise.
         */
        // TODO
        false
    }
    pub fn make_move(&mut self, c: Coord) -> Result<(),()> {
        /*
         * Updates struct with move if it is legal and returns Ok
         * Otherwise, return Err and do not update anything.
         * Additionally, if move wins the game, update the winner.
         */
        //TODO
        Ok(())
    }
    fn move_wins_game(&self, c: Coord) -> bool {
        // TODO
        false
    }
    pub fn new() -> Self {
        Self{ marks: HashMap::new(), turn: Mark::X, winner: None }
    }

    pub fn to_string(&self) -> String {
        // TODO
        "".to_string()
    }
}

