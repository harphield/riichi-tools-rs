/// Progressively calculates the chiitoitsu shanten of a hand.
/// It only matters how many tiles of a tile type are in the hand, but not which tile types those are.
/// Therefore when drawing or discarding the only additional input is how many tiles were in the hand before that action.
pub struct ChiitoiClassifier {
    shanten: i8,
}

impl ChiitoiClassifier {
    pub fn new(shanten: Option<i8>) -> ChiitoiClassifier {
        ChiitoiClassifier {
            shanten: shanten.unwrap_or(7),
        }
    }

    pub fn draw(&mut self, prev_tile_count: u8) {
        self.shanten -= ((prev_tile_count as i8 >> 1) ^ 1) & prev_tile_count as i8;
    }

    pub fn discard(&mut self, tile_count_after_discard: u8) {
        self.shanten +=
            ((tile_count_after_discard as i8 >> 1) ^ 1) & tile_count_after_discard as i8;
    }

    pub fn get_shanten(&self) -> i8 {
        self.shanten
    }
}
