use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
type Coord = (u32, u32);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mark{
    X,
    O,
}

struct HoboGame {
    marks: HashMap<Coord, Mark>,
    turn: Mark,
}

impl HoboGame {
    fn move_is_legal(&self, c: Coord) -> bool {
        // TODO
        false
    }
    fn make_move(&mut self, c: Coord) -> Result {
        //TODO
        Ok(())
    }
    fn move_wins_game(&self, c: Coord) -> bool {
        // TODO
        false
    }
    fn new() -> Self {
        Self{ marks: HashMap::new(), turn: Mark::X }
    }
}

