use crate::riichi::hand::Hand;
use crate::riichi::tile::Tile;
use crate::riichi::shapes::Shape;
use serde_json::{Map, Value};
use crate::riichi::riichi_error::RiichiError;
use crate::riichi::yaku::{YakuFinder, Yaku};
use crate::riichi::scores::Score;

/// Representation of the game state
pub struct Table {
    pub my_hand: Hand,
    my_riichi: bool,
    my_tsumo: bool,
    my_points: i32,
    // player to the right
    shimocha_discards: Vec<Tile>,
    shimocha_open_tiles: Vec<Shape>,
    shimocha_riichi: bool,
    shimocha_tsumo: bool,
    shimocha_points: i32,
    // player to the left
    kamicha_discards: Vec<Tile>,
    kamicha_open_tiles: Vec<Shape>,
    kamicha_riichi: bool,
    kamicha_tsumo: bool,
    kamicha_points: i32,
    // opposite player
    toimen_discards: Vec<Tile>,
    toimen_open_tiles: Vec<Shape>,
    toimen_riichi: bool,
    toimen_tsumo: bool,
    toimen_points: i32,

    // 1 = east, 2 = south, 3 = west, 4 = north
    prevalent_wind: u8,
    my_seat_wind: u8,
    wind_round: u8,
    total_round: u8,
    tiles_remaining: u8,

    dora_indicators: Vec<Tile>,

    riichi_sticks_in_pot: u8,
    tsumibo: u8,

}

impl Table {
    pub fn from_map(params: &Map<String, Value>) -> Result<Table, RiichiError> {
        let mut t = Table {
            my_hand: Default::default(),
            my_riichi: false,
            my_tsumo: false,
            my_points: 0,
            shimocha_discards: vec![],
            shimocha_open_tiles: vec![],
            shimocha_riichi: false,
            shimocha_tsumo: false,
            shimocha_points: 0,
            kamicha_discards: vec![],
            kamicha_open_tiles: vec![],
            kamicha_riichi: false,
            kamicha_tsumo: false,
            kamicha_points: 0,
            toimen_discards: vec![],
            toimen_open_tiles: vec![],
            toimen_riichi: false,
            toimen_tsumo: false,
            toimen_points: 0,
            prevalent_wind: 0,
            my_seat_wind: 0,
            wind_round: 0,
            total_round: 0,
            tiles_remaining: 128, // TODO how much is at the start?
            dora_indicators: vec![],
            riichi_sticks_in_pot: 0,
            tsumibo: 0
        };

        for (index, value) in params {
            if index.eq(&String::from("my_hand")) {
                match value {
                    Value::String(s) => {
                        match Hand::from_text(s, false) {
                            Ok(hand) => t.my_hand = hand,
                            Err(error) => return Err(error)
                        }

                    },
                    _ => ()
                }
            } else if index.eq(&String::from("my_riichi")) {
                match value {
                    Value::Bool(b) => {
                        t.my_riichi = *b;
                    },
                    _ => ()
                }
            } else if index.eq(&String::from("my_tsumo")) {
                match value {
                    Value::Bool(b) => {
                        t.my_tsumo = *b;
                    },
                    _ => ()
                }
            }
        }

        Ok(t)
    }

    pub fn am_i_oya(&self) -> bool {
        self.prevalent_wind > 0 && self.prevalent_wind == self.my_seat_wind
    }

    pub fn did_i_tsumo(&self) -> bool {
        self.my_tsumo
    }

    pub fn did_i_riichi(&self) -> bool {
        self.my_riichi
    }

    pub fn get_my_winning_tile(&self) -> Tile {
        self.my_hand.get_drawn_tile().clone()
    }

    pub fn get_tiles_remaining(&self) -> u8 {
        self.tiles_remaining
    }

    pub fn get_my_seat_wind(&self) -> u8 {
        self.my_seat_wind
    }

    pub fn get_prevalent_wind(&self) -> u8 {
        self.prevalent_wind
    }

    pub fn yaku(&mut self) -> Option<(Vec<Yaku>, Score)> {
        let yf = YakuFinder::new();
        yf.find(self)
    }
}

mod tests {
    use super::*;

    #[test]
    fn set_my_hand() {

    }
}
