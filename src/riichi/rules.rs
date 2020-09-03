pub enum GameLength {
    Tonpuusen,
    Hanchan,
}

pub struct Rules {
    game_length: GameLength,
    aka_ari: bool,
    kuitan_ari: bool,
    // TODO more rules
}
