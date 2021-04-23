use crate::riichi::fast_hand_calculator::chiitoi_classifier::ChiitoiClassifier;
use crate::riichi::fast_hand_calculator::kokushi_classifier::KokushiClassifier;
use crate::riichi::fast_hand_calculator::progressive_honor_classifier::ProgressiveHonorClassifier;
use crate::riichi::fast_hand_calculator::suit_classifier::SuitClassifier;
use crate::riichi::riichi_error::RiichiError;
use crate::riichi::tile::{Tile, TileType, TileColor};
use crate::riichi::hand::Hand;

static BASE5TABLE: [u32; 9] = [1, 5, 25, 125, 625, 3125, 15625, 78125, 390625];

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

    fn init(&mut self, hand: &Hand) {
        for tile_o in hand.get_tiles() {
            match tile_o {
                None => {}
                Some(tile) => {
                    self.in_hand_by_type[tile.to_id() as usize] += 1;
                    let prev_tile_count = self.concealed_tiles[tile.to_id() as usize];
                    self.concealed_tiles[tile.to_id() as usize] += 1;

                    self.kokushi.draw(tile.to_id(), prev_tile_count);
                    self.chiitoi.draw(prev_tile_count);

                    match tile.tile_type {
                        TileType::Number(value, color) => {
                            match color {
                                TileColor::Manzu => self.base5hashes[0] += BASE5TABLE[value as usize],
                                TileColor::Pinzu => self.base5hashes[1] += BASE5TABLE[value as usize],
                                TileColor::Souzu => self.base5hashes[2] += BASE5TABLE[value as usize],
                            }
                        }
                        TileType::Wind(value) | TileType::Dragon(value) => {
                            self.arrangement_values[3] = self.honor_classifier.draw(prev_tile_count, self.jihai_meld_bit >> value & 1)
                        }
                    }
                }
            }
        }

        self.update_value(0);
        self.update_value(1);
        self.update_value(2);
    }

    fn update_value(&mut self, suit: usize) {
        self.arrangement_values[suit] = self.suit_classifiers[suit].get_value(&self.concealed_tiles, suit, &self.base5hashes);
    }

    pub fn calculate_shanten(&self, arrangement_values: Vec<u8>) -> Result<i8, RiichiError> {
        // let shanten = ArrangementClassifier.Classify(arrangement_values);
        // if self.meld_count > 0 {
        //     return shanten;
        // }
        //
        // // return Math.Min(shanten, Math.Min(_kokushi.Shanten, _chiitoi.Shanten));
        // Ok(*shantens.iter().min().unwrap())
        Err(RiichiError::new(404, "test"))
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
            chiitoi: ChiitoiClassifier::new(),
            kokushi: KokushiClassifier::new(),
            honor_classifier: ProgressiveHonorClassifier::new(),
            meld_count: 0,
        }
    }
}
