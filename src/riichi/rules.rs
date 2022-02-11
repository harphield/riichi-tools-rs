/// Length of the game
pub enum GameLength {
    /// 1 round
    Tonpuusen,
    /// 2 rounds
    Hanchan,
}

/// All rules will go here
pub struct Rules {
    /// How long is the game?
    pub game_length: GameLength,
    /// Do we use red 5s?
    pub aka_ari: bool,
    /// Do we allow open tanyao?
    pub kuitan_ari: bool,
    // TODO more rules
}
