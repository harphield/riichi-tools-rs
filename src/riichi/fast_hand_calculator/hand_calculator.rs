use crate::riichi::fast_hand_calculator::arrangement_classifier::ArrangementClassifier;
use crate::riichi::fast_hand_calculator::chiitoi_classifier::ChiitoiClassifier;
use crate::riichi::fast_hand_calculator::kokushi_classifier::KokushiClassifier;
use crate::riichi::fast_hand_calculator::progressive_honor_classifier::ProgressiveHonorClassifier;
use crate::riichi::fast_hand_calculator::suit_classifier::SuitClassifier;
use crate::riichi::hand::Hand;
use crate::riichi::tile::{Tile, TileColor, TileType};
use crate::riichi::shapes::{OpenShape, OpenKan, CompleteShape, ClosedShape};

/// the 0 index is not used
static BASE5TABLE: [u32; 10] = [0, 1, 5, 25, 125, 625, 3125, 15625, 78125, 390625];

pub struct HandCalculator {
    arrangement_values: [u32; 4],
    /// base 5 representation of concealed suits. Not relevant with a meld.
    base5hashes: [u32; 3],
    concealed_tiles: [u8; 34],
    /// tiles in hand by tile type, including melds, kan is 4 tiles here
    in_hand_by_type: [u8; 34],
    /// non-honors, identified by meldId, youngest meld in least significant bits
    melds: [u8; 3],
    /// bit=1 for honor pon, least significant bit represents east wind. bit=0 for both kan and no meld.
    jihai_meld_bit: u8,
    suit_classifiers: [SuitClassifier; 3],
    chiitoi: ChiitoiClassifier,
    kokushi: KokushiClassifier,
    honor_classifier: ProgressiveHonorClassifier,
    meld_count: u8,
}

impl HandCalculator {
    pub fn new() -> HandCalculator {
        HandCalculator {
            ..Default::default()
        }
    }

    pub fn init(&mut self, hand: &Hand) {
        for tile_o in hand.get_tiles() {
            match tile_o {
                None => {}
                Some(tile) => {
                    if !tile.is_open && !tile.is_kan {
                        self.in_hand_by_type[tile.to_id_minus_1() as usize] += 1;
                        let prev_tile_count = self.concealed_tiles[tile.to_id_minus_1() as usize];
                        self.concealed_tiles[tile.to_id_minus_1() as usize] += 1;

                        self.kokushi
                            .draw(tile.to_id_minus_1() as u32, prev_tile_count as u32);
                        self.chiitoi.draw(prev_tile_count);

                        match tile.tile_type {
                            TileType::Number(value, color) => match color {
                                TileColor::Manzu => self.base5hashes[0] += BASE5TABLE[value as usize],
                                TileColor::Pinzu => self.base5hashes[1] += BASE5TABLE[value as usize],
                                TileColor::Souzu => self.base5hashes[2] += BASE5TABLE[value as usize],
                            },
                            TileType::Wind(value) | TileType::Dragon(value) => {
                                self.arrangement_values[3] = self
                                    .honor_classifier
                                    .draw(prev_tile_count, self.jihai_meld_bit >> (value - 1) & 1);
                            }
                        }
                    }
                }
            }
        }

        for shape in hand.get_open_shapes() {
            match shape {
                OpenShape::Chi(tiles) => {
                    self.in_hand_by_type[tiles[0].to_id_minus_1() as usize] += 1;
                    self.in_hand_by_type[tiles[1].to_id_minus_1() as usize] += 1;
                    self.in_hand_by_type[tiles[2].to_id_minus_1() as usize] += 1;
                    self.concealed_tiles[tiles[0].to_id_minus_1() as usize] += 1;
                    self.concealed_tiles[tiles[1].to_id_minus_1() as usize] += 1;
                    self.concealed_tiles[tiles[2].to_id_minus_1() as usize] += 1;

                    let called_tile = tiles.iter().find(|t| t.called_from != 0).unwrap();
                    self.in_hand_by_type[called_tile.to_id_minus_1() as usize] -= 1;
                    self.concealed_tiles[called_tile.to_id_minus_1() as usize] -= 1;

                    self.chii(&tiles[0], &called_tile);
                }
                OpenShape::Pon(tiles) => {
                    for _i in 0..2 {
                        let prev_tile_count = self.concealed_tiles[tiles[0].to_id_minus_1() as usize];
                        self.in_hand_by_type[tiles[0].to_id_minus_1() as usize] += 1;
                        self.concealed_tiles[tiles[0].to_id_minus_1() as usize] += 1;

                        match tiles[0].tile_type {
                            TileType::Number(_, _) => {},
                            TileType::Wind(value) | TileType::Dragon(value) => {
                                self.arrangement_values[3] = self
                                    .honor_classifier
                                    .draw(prev_tile_count, self.jihai_meld_bit >> (value - 1) & 1);
                            }
                        }
                    }

                    self.pon(&tiles[0]);
                }
                OpenShape::Kan(open_kan) => {
                    match open_kan {
                        OpenKan::Daiminkan(tiles) => {
                            for _i in 0..3 {
                                let prev_tile_count = self.concealed_tiles[tiles[0].to_id_minus_1() as usize];
                                self.in_hand_by_type[tiles[0].to_id_minus_1() as usize] += 1;
                                self.concealed_tiles[tiles[0].to_id_minus_1() as usize] += 1;

                                match tiles[0].tile_type {
                                    TileType::Number(_, _) => {},
                                    TileType::Wind(value) | TileType::Dragon(value) => {
                                        self.arrangement_values[3] = self
                                            .honor_classifier
                                            .draw(prev_tile_count, self.jihai_meld_bit >> (value - 1) & 1);
                                    }
                                }
                            }

                            self.daiminkan(&tiles[0]);
                        }
                        OpenKan::Shouminkan(tiles) => {
                            // first we pon, then shouminkan
                            for _i in 0..2 {
                                let prev_tile_count = self.concealed_tiles[tiles[0].to_id_minus_1() as usize];
                                self.in_hand_by_type[tiles[0].to_id_minus_1() as usize] += 1;
                                self.concealed_tiles[tiles[0].to_id_minus_1() as usize] += 1;

                                match tiles[0].tile_type {
                                    TileType::Number(_, _) => {},
                                    TileType::Wind(value) | TileType::Dragon(value) => {
                                        self.arrangement_values[3] = self
                                            .honor_classifier
                                            .draw(prev_tile_count, self.jihai_meld_bit >> (value - 1) & 1);
                                    }
                                }
                            }

                            self.pon(&tiles[0]);

                            let prev_tile_count = self.concealed_tiles[tiles[0].to_id_minus_1() as usize];
                            self.in_hand_by_type[tiles[0].to_id_minus_1() as usize] += 1;
                            self.concealed_tiles[tiles[0].to_id_minus_1() as usize] += 1;

                            match tiles[0].tile_type {
                                TileType::Number(_, _) => {},
                                TileType::Wind(value) | TileType::Dragon(value) => {
                                    self.arrangement_values[3] = self
                                        .honor_classifier
                                        .draw(prev_tile_count, self.jihai_meld_bit >> (value - 1) & 1);
                                }
                            }

                            self.shouminkan(&tiles[0]);
                        }
                    }
                }
            }
        }

        for shape in hand.get_shapes().iter() {
            match shape {
                CompleteShape::Closed(closed_shape) => {
                    match closed_shape {
                        ClosedShape::Kantsu(tiles) => {
                            for _i in 0..4 {
                                let prev_tile_count = self.concealed_tiles[tiles[0].to_id_minus_1() as usize];
                                self.in_hand_by_type[tiles[0].to_id_minus_1() as usize] += 1;
                                self.concealed_tiles[tiles[0].to_id_minus_1() as usize] += 1;

                                match tiles[0].tile_type {
                                    TileType::Number(_, _) => {},
                                    TileType::Wind(value) | TileType::Dragon(value) => {
                                        self.arrangement_values[3] = self
                                            .honor_classifier
                                            .draw(prev_tile_count, self.jihai_meld_bit >> (value - 1) & 1);
                                    }
                                }
                            }

                            self.ankan(&tiles[0]);
                        }
                        _ => {}
                    }
                }
                CompleteShape::Open(_) => {}
            }
        }

        self.update_value(0);
        self.update_value(1);
        self.update_value(2);
    }

    pub fn draw(&mut self, tile: &Tile) {
        if self.tiles_in_hand() != 13 {
            panic!("Can only draw with a 13 tile hand.");
        }

        if self.in_hand_by_type[tile.to_id_minus_1() as usize] == 4 {
            panic!("Can't draw a tile with 4 of that tile in hand.");
        }

        self.in_hand_by_type[tile.to_id_minus_1() as usize] += 1;
        let prev_tile_count = self.concealed_tiles[tile.to_id_minus_1() as usize];
        self.concealed_tiles[tile.to_id_minus_1() as usize] += 1;

        self.kokushi
            .draw(tile.to_id_minus_1() as u32, prev_tile_count as u32);
        self.chiitoi.draw(prev_tile_count);

        match tile.tile_type {
            TileType::Number(value, color) => match color {
                TileColor::Manzu => {
                    self.base5hashes[0] += BASE5TABLE[value as usize];
                    self.update_value(0);
                }
                TileColor::Pinzu => {
                    self.base5hashes[1] += BASE5TABLE[value as usize];
                    self.update_value(1);
                }
                TileColor::Souzu => {
                    self.base5hashes[2] += BASE5TABLE[value as usize];
                    self.update_value(2);
                }
            },
            TileType::Wind(value) | TileType::Dragon(value) => {
                self.arrangement_values[3] = self
                    .honor_classifier
                    .draw(prev_tile_count, self.jihai_meld_bit >> (value - 1) & 1);
            }
        }
    }

    pub fn discard(&mut self, tile: &Tile) {
        if self.tiles_in_hand() != 13 {
            panic!("Can only draw with a 13 tile hand.");
        }

        if self.in_hand_by_type[tile.to_id_minus_1() as usize] == 4 {
            panic!("Can't draw a tile with 4 of that tile in hand.");
        }
        self.in_hand_by_type[tile.to_id_minus_1() as usize] -= 1;
        self.concealed_tiles[tile.to_id_minus_1() as usize] -= 1;
        let tile_count_after_discard = self.concealed_tiles[tile.to_id_minus_1() as usize];

        self.kokushi
            .discard(tile.to_id_minus_1() as u32, tile_count_after_discard as u32);
        self.chiitoi.discard(tile_count_after_discard);

        match tile.tile_type {
            TileType::Number(value, color) => match color {
                TileColor::Manzu => {
                    self.base5hashes[0] -= BASE5TABLE[value as usize];
                    self.update_value(0);
                }
                TileColor::Pinzu => {
                    self.base5hashes[1] -= BASE5TABLE[value as usize];
                    self.update_value(1);
                }
                TileColor::Souzu => {
                    self.base5hashes[2] -= BASE5TABLE[value as usize];
                    self.update_value(2);
                }
            },
            TileType::Wind(value) | TileType::Dragon(value) => {
                self.arrangement_values[3] = self
                    .honor_classifier
                    .discard(tile_count_after_discard, self.jihai_meld_bit >> (value - 1) & 1);
            }
        }
    }

    pub fn chii(&mut self, lowest_tile: &Tile, called_tile: &Tile) {
        if self.tiles_in_hand() != 13 {
            // panic!("Chii only after discard.");
        }

        match lowest_tile.tile_type {
            TileType::Wind(_) | TileType::Dragon(_) => panic!("Not a valid suit for chii"),
            _ => {}
        }

        if lowest_tile.to_id_minus_1() != called_tile.to_id_minus_1() {
            self.concealed_tiles[lowest_tile.to_id_minus_1() as usize] -= 1;
        }
        if lowest_tile.to_id_minus_1() + 1 != called_tile.to_id_minus_1() {
            self.concealed_tiles[(lowest_tile.to_id_minus_1() + 1) as usize] -= 1;
        }
        if lowest_tile.to_id_minus_1() + 2 != called_tile.to_id_minus_1() {
            self.concealed_tiles[(lowest_tile.to_id_minus_1() + 2) as usize] -= 1;
        }

        match lowest_tile.tile_type {
            TileType::Number(value, color) => match color {
                TileColor::Manzu => {
                    self.melds[0] <<= 6;
                    self.melds[0] += 1 + (value - 1);

                    self.suit_classifiers[0].set_melds(self.melds[0]);
                    self.update_value(0);
                }
                TileColor::Pinzu => {
                    self.melds[1] <<= 6;
                    self.melds[1] += 1 + (value - 1);

                    self.suit_classifiers[1].set_melds(self.melds[1]);
                    self.update_value(1);
                }
                TileColor::Souzu => {
                    self.melds[2] <<= 6;
                    self.melds[2] += 1 + (value - 1);

                    self.suit_classifiers[2].set_melds(self.melds[2]);
                    self.update_value(2);
                }
            },
            _ => {}
        }

        self.meld_count += 1;
        self.in_hand_by_type[called_tile.to_id_minus_1() as usize] += 1;
    }

    pub fn pon(&mut self, tile: &Tile) {
        if self.tiles_in_hand() != 13 {
            // panic!("Pon only after discard.");
        }

        let prev_tiles = self.concealed_tiles[tile.to_id_minus_1() as usize];
        self.concealed_tiles[tile.to_id_minus_1() as usize] -= 2;

        match tile.tile_type {
            TileType::Number(value, color) => match color {
                TileColor::Manzu => {
                    self.melds[0] <<= 6;
                    self.melds[0] += 1 + 7 + (value - 1);

                    self.suit_classifiers[0].set_melds(self.melds[0]);
                    self.update_value(0);
                }
                TileColor::Pinzu => {
                    self.melds[1] <<= 6;
                    self.melds[1] += 1 + 7 + (value - 1);

                    self.suit_classifiers[1].set_melds(self.melds[1]);
                    self.update_value(1);
                }
                TileColor::Souzu => {
                    self.melds[2] <<= 6;
                    self.melds[2] += 1 + 7 + (value - 1);

                    self.suit_classifiers[2].set_melds(self.melds[2]);
                    self.update_value(2);
                }
            },
            TileType::Wind(_) | TileType::Dragon(_) => {
                self.arrangement_values[3] = self
                    .honor_classifier
                    .pon(prev_tiles);
                self.jihai_meld_bit += 1 << tile.get_value();
            }
        }

        self.meld_count += 1;
        self.in_hand_by_type[tile.to_id_minus_1() as usize] += 1;
    }

    pub fn shouminkan(&mut self, tile: &Tile) {
        if self.tiles_in_hand() != 14 {
            // panic!("Shouminkan only after draw.");
        }

        self.concealed_tiles[tile.to_id_minus_1() as usize] -= 1;

        match tile.tile_type {
            TileType::Number(value, color) => {
                let c: usize = match color {
                    TileColor::Manzu => 0,
                    TileColor::Pinzu => 1,
                    TileColor::Souzu => 2,
                };

                let pon = 1 + 7 + (value - 1);
                for i in 0..4 {
                    if (self.melds[c] >> 6 * i & 0b111111) == pon {
                        self.melds[c] += 9 << 6 * i;
                        break;
                    }
                }

                self.suit_classifiers[c].set_melds(self.melds[c]);
                self.update_value(c);
            }
            TileType::Wind(_) | TileType::Dragon(_) => {
                self.arrangement_values[3] = self.honor_classifier.shouminkan();
            }
        }
    }

    pub fn ankan(&mut self, tile: &Tile) {
        if self.tiles_in_hand() != 14 {
            // panic!("Ankan only after draw.");
        }

        self.concealed_tiles[tile.to_id_minus_1() as usize] -= 4;
        self.meld_count += 1;

        match tile.tile_type {
            TileType::Number(value, color) => {
                let c: usize = match color {
                    TileColor::Manzu => 0,
                    TileColor::Pinzu => 1,
                    TileColor::Souzu => 2,
                };

                self.melds[c] <<= 6;
                self.melds[c] += 1 + 7 + 9 + (value - 1);
                self.suit_classifiers[c].set_melds(self.melds[c]);
                self.update_value(c);
            }
            TileType::Wind(_) | TileType::Dragon(_) => {
                self.arrangement_values[3] = self.honor_classifier.ankan();
            }
        }
    }

    pub fn daiminkan(&mut self, tile: &Tile) {
        if self.tiles_in_hand() != 13 {
            // panic!("Daiminkan only after discard.");
        }

        self.in_hand_by_type[tile.to_id_minus_1() as usize] += 1;
        self.concealed_tiles[tile.to_id_minus_1() as usize] -= 3;
        self.meld_count += 1;

        match tile.tile_type {
            TileType::Number(value, color) => {
                let c: usize = match color {
                    TileColor::Manzu => 0,
                    TileColor::Pinzu => 1,
                    TileColor::Souzu => 2,
                };

                self.melds[c] <<= 6;
                self.melds[c] += 1 + 7 + 9 + (value - 1);
                self.suit_classifiers[c].set_melds(self.melds[c]);
                self.update_value(c);
            }
            TileType::Wind(_) | TileType::Dragon(_) => {
                self.arrangement_values[3] = self.honor_classifier.daiminkan();
            }
        }
    }

    /// 34 ints, one per tile_id.
    /// -1 if that tile_id is not an ukeIre.
    /// 0-4 for the remaining tiles of that tile_id if ukeIre.
    pub fn get_uke_ire_for_13(&mut self) -> [i32; 34] {
        if self.tiles_in_hand() != 13 {
            panic!("It says 13 in the method name!");
        }

        let current_shanten = self.calculate_shanten(&self.arrangement_values);

        let mut uke_ire: [i32; 34] = [-1; 34];
        let mut tile_id = 0;
        let mut local_arrangements = [
            self.arrangement_values[0],
            self.arrangement_values[1],
            self.arrangement_values[2],
            self.arrangement_values[3],
        ];

        for suit in 0..3 {
            for value in 0..9 {
                if self.in_hand_by_type[tile_id] != 4 {
                    self.kokushi
                        .draw(tile_id as u32, self.concealed_tiles[tile_id] as u32);
                    self.chiitoi.draw(self.concealed_tiles[tile_id]);

                    self.concealed_tiles[tile_id] += 1;
                    self.base5hashes[suit] += BASE5TABLE[value];
                    local_arrangements[suit] = self.suit_classifiers[suit].get_value(
                        &self.concealed_tiles,
                        suit,
                        &self.base5hashes,
                    );

                    let new_shanten = self.calculate_shanten(&local_arrangements);
                    let delta = current_shanten - new_shanten;
                    uke_ire[tile_id] =
                        ((5 - self.in_hand_by_type[tile_id]) as i8 * delta - 1) as i32;

                    self.concealed_tiles[tile_id] -= 1;
                    self.base5hashes[suit] -= BASE5TABLE[value];

                    self.kokushi
                        .discard(tile_id as u32, self.concealed_tiles[tile_id] as u32);
                    self.chiitoi.discard(self.concealed_tiles[tile_id]);
                }

                tile_id += 1;
            }

            local_arrangements[suit] = self.arrangement_values[suit];
        }

        for value in 0..7 {
            if self.in_hand_by_type[tile_id] != 4 {
                self.kokushi
                    .draw(tile_id as u32, self.concealed_tiles[tile_id] as u32);
                self.chiitoi.draw(self.concealed_tiles[tile_id]);

                // TODO clone the honor_classifier?
                local_arrangements[3] = self.honor_classifier.draw(
                    self.concealed_tiles[tile_id],
                    self.jihai_meld_bit >> value & 1,
                );

                let new_shanten = self.calculate_shanten(&local_arrangements);
                let delta = current_shanten - new_shanten;
                uke_ire[tile_id] = ((5 - self.in_hand_by_type[tile_id]) as i8 * delta - 1) as i32;

                self.kokushi
                    .discard(tile_id as u32, self.concealed_tiles[tile_id] as u32);
                self.chiitoi.discard(self.concealed_tiles[tile_id]);
            }

            tile_id += 1;
        }

        uke_ire
    }

    fn tiles_in_hand(&self) -> u8 {
        self.concealed_tiles.iter().sum::<u8>() + self.meld_count * 3
    }

    fn update_value(&mut self, suit: usize) {
        self.arrangement_values[suit] =
            self.suit_classifiers[suit].get_value(&self.concealed_tiles, suit, &self.base5hashes);
    }

    fn calculate_shanten(&self, arrangement_values: &[u32; 4]) -> i8 {
        let shanten = ArrangementClassifier::new().classify(&arrangement_values);
        if self.meld_count > 0 {
            return shanten as i8;
        }

        let shantens = [
            shanten as i8,
            self.chiitoi.get_shanten(),
            self.kokushi.get_shanten(),
        ];

        *shantens.iter().min().unwrap()
    }

    pub fn shanten(&self) -> i8 {
        self.calculate_shanten(&self.arrangement_values) - 1
    }
}

impl Default for HandCalculator {
    fn default() -> HandCalculator {
        HandCalculator {
            arrangement_values: [0; 4],
            base5hashes: [0; 3],
            concealed_tiles: [0; 34],
            in_hand_by_type: [0; 34],
            melds: [0; 3],
            jihai_meld_bit: 0,
            suit_classifiers: [
                SuitClassifier::new(),
                SuitClassifier::new(),
                SuitClassifier::new(),
            ],
            chiitoi: ChiitoiClassifier::new(None),
            kokushi: KokushiClassifier::new(None, None),
            honor_classifier: ProgressiveHonorClassifier::new(),
            meld_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::riichi::fast_hand_calculator::hand_calculator::HandCalculator;
    use crate::riichi::hand::Hand;

    #[test]
    fn pinfu_tenpai() {
        let hand = Hand::from_text("123456789m23p11s", false).unwrap();

        let mut hc = HandCalculator::new();
        hc.init(&hand);

        let shanten = hc.shanten();

        assert_eq!(shanten, 0);
    }

    #[test]
    fn chiitoitsu_tenpai() {
        let hand = Hand::from_text("1133557799p22s3z", false).unwrap();

        let mut hc = HandCalculator::new();
        hc.init(&hand);

        let shanten = hc.shanten();

        assert_eq!(shanten, 0);
    }

    #[test]
    fn kokushi_tenpai_13_waits() {
        let hand = Hand::from_text("19m19s19p1234567z", false).unwrap();

        let mut hc = HandCalculator::new();
        hc.init(&hand);

        let shanten = hc.shanten();

        assert_eq!(shanten, 0);
    }

    #[test]
    fn tenpai_with_chi() {
        let hand = Hand::from_text("123456m11p222s(789p1)", false).unwrap();

        let mut hc = HandCalculator::new();
        hc.init(&hand);

        let shanten = hc.shanten();

        assert_eq!(shanten, -1);
    }

    #[test]
    fn iishanten_hand_with_unkanned_4_tiles() {
        let hand = Hand::from_text("1111m222s333p444z", false).unwrap();

        let mut hc = HandCalculator::new();
        hc.init(&hand);

        let shanten = hc.shanten();

        assert_eq!(shanten, 1);
    }
}
