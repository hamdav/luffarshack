use std::collections::HashMap;

type Coord = (u32, u32);

enum Mark{
    X,
    O,
}

struct HoboGame {
    marks: HashMap<Coord, Mark>,
    turn: Mark,
}
