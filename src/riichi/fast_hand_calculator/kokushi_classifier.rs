/// Progressively calculates the Shanten for Kokushi.
/// It is expected to only be called for changes to terminal or honor tiles.
/// It only matters how many tiles of a tile type are in the hand, but not which tile types those are (aside from them being terminals or honors).
/// Therefore when drawing or discarding the only additional input is how many tiles were in the hand before that action.
pub struct KokushiClassifier {
    shanten: i8,
    pairs: u8,
}

impl KokushiClassifier {
    pub fn new(shanten: Option<i8>, pairs: Option<u8>) -> KokushiClassifier {
        KokushiClassifier {
            shanten: match shanten {
                None => 14,
                Some(s) => s,
            },
            pairs: match pairs {
                None => 1,
                Some(p) => p,
            },
        }
    }

    pub fn draw(&mut self, tile_id: u32, prev_tile_count: u32) {
        // (1 << x & 0b100000001100000001100000001) >> x | (x + 5) >> 5
        // 1 if the tileType is a terminal or honor, else 0

        let r = (1u64 << tile_id & 0b100000001100000001100000001) >> tile_id
            | (tile_id as u64 + 5) >> 5;

        // TODO Is suspect this can be simplified

        // 1 if previousTileCount < 2, else 0
        let s = (prev_tile_count as u64 ^ 2) >> 1 & r;
        // 1 if previousTileCount == 1, else 0
        let p = prev_tile_count as u64 & s;
        // 1 if no pair was added or there were no pairs before, else 0
        let t = (self.pairs as u64 | !p) & s;
        self.pairs <<= p;
        self.shanten -= t as i8;
    }

    pub fn discard(&mut self, tile_id: u32, tile_count_after_discard: u32) {
        // (1 << x & 0b100000001100000001100000001) >> x | (x + 5) >> 5
        // 1 if the tileType is a terminal or honor, else 0
        let r = (1u64 << tile_id & 0b100000001100000001100000001) >> tile_id
            | (tile_id as u64 + 5) >> 5;

        // 1 if tileCountAfterDiscard < 2, else 0
        let s = (tile_count_after_discard as u64 ^ 2) >> 1 & r;
        // 1 if tileCountAfterDiscard == 1, else 0
        let p = tile_count_after_discard as u64 & s;
        self.pairs >>= p;
        // 1 if no pair was removed or there were at least two pairs before, else 0
        let t = (self.pairs as u64 | !p) & s;
        self.shanten += t as i8;
    }

    pub fn get_shanten(&self) -> i8 {
        self.shanten
    }
}
