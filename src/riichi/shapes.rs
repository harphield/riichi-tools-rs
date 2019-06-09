pub enum Shape {
    Complete(CompleteShape),
    Incomplete(IncompleteShape)
}

enum CompleteShape {
    Shuntsu,
    Koutsu,
    Toitsu,
    Single
}

enum IncompleteShape {
    Ryanmen([Tile, 2]),
    Shanpon([Tile, 2]),
    Tanki(Tile)
}