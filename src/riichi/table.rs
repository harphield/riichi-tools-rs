use crate::riichi::hand::Hand;
use crate::riichi::tile::Tile;
use crate::riichi::shapes::Shape;
use serde_json::{Map, Value};
use crate::riichi::riichi_error::RiichiError;
use crate::riichi::yaku::{YakuFinder, Yaku};
use crate::riichi::scores::Score;
use crate::riichi::rules::Rules;

/// Representation of the game state
pub struct Table {
    my_hand: Option<Hand>,
    my_discards: Vec<Tile>,
    my_riichi: Option<bool>,
    my_tsumo: Option<bool>,
    my_points: Option<i32>,
    // player to the right (shimocha)
    p1_discards: Vec<Tile>,
    p1_open_tiles: Vec<Shape>,
    p1_riichi: Option<bool>,
    p1_tsumo: Option<bool>,
    p1_points: Option<i32>,
    // opposite player (toimen)
    p2_discards: Vec<Tile>,
    p2_open_tiles: Vec<Shape>,
    p2_riichi: Option<bool>,
    p2_tsumo: Option<bool>,
    p2_points: Option<i32>,
    // player to the left (kamicha)
    p3_discards: Vec<Tile>,
    p3_open_tiles: Vec<Shape>,
    p3_riichi: Option<bool>,
    p3_tsumo: Option<bool>,
    p3_points: Option<i32>,

    // 1 = east, 2 = south, 3 = west, 4 = north
    prevalent_wind: Option<u8>,
    my_seat_wind: Option<u8>,
    wind_round: Option<u8>,
    total_round: Option<u8>,
    tiles_remaining: Option<u8>,

    riichi_sticks_in_pot: Option<u8>,
    tsumibo: Option<u8>,

    dora_indicators: Vec<Tile>,
    visible_tiles: Vec<Tile>,

    rules: Option<Rules>,
}

impl Table {
    pub fn from_map(params: &Map<String, Value>) -> Result<Table, RiichiError> {
        let mut t = Table {
            my_hand: None,
            my_discards: vec![],
            my_riichi: None,
            my_tsumo: None,
            my_points: None,
            p1_discards: vec![],
            p1_open_tiles: vec![],
            p1_riichi: None,
            p1_tsumo: None,
            p1_points: None,
            p2_discards: vec![],
            p2_open_tiles: vec![],
            p2_riichi: None,
            p2_tsumo: None,
            p2_points: None,
            p3_discards: vec![],
            p3_open_tiles: vec![],
            p3_riichi: None,
            p3_tsumo: None,
            p3_points: None,
            prevalent_wind: None,
            my_seat_wind: None,
            wind_round: None,
            total_round: None,
            tiles_remaining: None,
            riichi_sticks_in_pot: None,
            tsumibo: None,
            dora_indicators: vec![],
            visible_tiles: vec![],
            rules: None,
        };

        for (index, value) in params {
            if index.eq(&String::from("my_hand")) {
                match value {
                    Value::String(s) => {
                        match Hand::from_text(s, false) {
                            Ok(hand) => t.my_hand = Some(hand),
                            Err(error) => return Err(error)
                        }

                    },
                    _ => ()
                }
            } else if index.eq(&String::from("my_riichi")) {
                match value {
                    Value::Bool(b) => {
                        t.my_riichi = Some(*b);
                    },
                    _ => ()
                }
            } else if index.eq(&String::from("my_tsumo")) {
                match value {
                    Value::Bool(b) => {
                        t.my_tsumo = Some(*b);
                    },
                    _ => ()
                }
            } else if index.eq(&String::from("prevalent_wind")) {
                match value {
                    Value::Number(v) => {
                        t.prevalent_wind = Some(v.as_u64().unwrap() as u8);
                    },
                    Value::String(v) => {
                        let number: u8 = v.parse().unwrap();
                        t.prevalent_wind = Some(number);
                    }
                    _ => ()
                }
            } else if index.eq(&String::from("my_seat_wind")) {
                match value {
                    Value::Number(v) => {
                        t.my_seat_wind = Some(v.as_u64().unwrap() as u8);
                    },
                    Value::String(v) => {
                        let number: u8 = v.parse().unwrap();
                        t.my_seat_wind = Some(number);
                    }
                    _ => ()
                }
            }
        }

        Ok(t)
    }

    pub fn set_seat(&mut self, seat: u8) {
        self.my_seat_wind = Some(seat);
    }

    pub fn am_i_oya(&self) -> bool {
        match self.my_seat_wind {
            None => false,
            Some(seat) => {
                seat == 1
            },
        }
    }

    pub fn set_my_tsumo(&mut self, value: bool) {
        self.my_tsumo = Some(value);
    }
    pub fn set_p1_tsumo(&mut self, value: bool) {
        self.p1_tsumo = Some(value);
    }
    pub fn set_p2_tsumo(&mut self, value: bool) {
        self.p2_tsumo = Some(value);
    }
    pub fn set_p3_tsumo(&mut self, value: bool) {
        self.p3_tsumo = Some(value);
    }

    pub fn did_i_tsumo(&self) -> bool {
        match self.my_tsumo {
            None => false,
            Some(value) => value,
        }
    }

    pub fn set_my_riichi(&mut self, value: bool) {
        self.my_riichi = Some(value);
    }
    pub fn set_p1_riichi(&mut self, value: bool) {
        self.p1_riichi = Some(value);
    }
    pub fn set_p2_riichi(&mut self, value: bool) {
        self.p2_riichi = Some(value);
    }
    pub fn set_p3_riichi(&mut self, value: bool) {
        self.p3_riichi = Some(value);
    }

    pub fn did_i_riichi(&self) -> bool {
        match &self.my_riichi {
            None => false,
            Some(value) => *value,
        }
    }

    pub fn set_my_hand(&mut self, hand: Hand) {
        self.my_hand = Some(hand);
    }

    pub fn get_my_hand(&mut self) -> &mut Hand {
        match &mut self.my_hand {
            None => panic!("No hand!"),
            Some(hand) => hand,
        }
    }

    pub fn get_my_winning_tile(&self) -> Tile {
        match &self.my_hand {
            None => panic!("No drawn tile in hand!"),
            Some(hand) => {
                hand.get_drawn_tile().unwrap().clone()
            },
        }
    }

    pub fn set_tiles_remaining(&mut self, value: u8) {
        self.tiles_remaining = Some(value);
    }

    pub fn get_tiles_remaining(&self) -> Option<u8> {
        self.tiles_remaining
    }

    pub fn set_my_seat_wind(&mut self, value: u8) {
        self.my_seat_wind = Some(value);
    }

    pub fn get_my_seat_wind(&self) -> Option<u8> {
        self.my_seat_wind
    }

    pub fn set_prevalent_wind(&mut self, value: u8) {
        self.prevalent_wind = Some(value);
    }

    pub fn get_prevalent_wind(&self) -> Option<u8> {
        self.prevalent_wind
    }

    pub fn set_dora_indicators(&mut self, indicators: Vec<Tile>) {
        self.dora_indicators = indicators;
    }

    pub fn add_dora_indicator(&mut self, indicator: Tile) {
        self.dora_indicators.push(indicator);
        self.visible_tiles.push(indicator);
    }

    pub fn get_dora_indicators(&self) -> &Vec<Tile> {
        &self.dora_indicators
    }

    pub fn add_tile_to_visible_tiles(&mut self, tile: Tile) {
        self.visible_tiles.push(tile);
    }

    pub fn add_vector_to_visible_tiles(&mut self, mut tiles: Vec<Tile>) {
        self.visible_tiles.append(&mut tiles);
    }

    pub fn reset_visible_tiles(&mut self) {
        self.visible_tiles = vec![];
    }

    pub fn get_visible_tiles(&self) -> &Vec<Tile> {
        &self.visible_tiles
    }

    pub fn get_visible_tiles_as_array_34(&self) -> [u8; 34] {
        let mut result = [0; 34];

        for tile in &self.visible_tiles {
            result[(tile.to_id() - 1) as usize] += 1;

            if result[(tile.to_id() - 1) as usize] > 4 {
                panic!("can not have more than 4 tiles!!!");
            }
        }

        result
    }

    pub fn set_total_round(&mut self, value: u8) {
        self.total_round = Some(value);
    }

    pub fn get_total_round(&self) -> Option<u8> {
        self.total_round
    }

    pub fn set_riichi_sticks(&mut self, value: u8) {
        self.riichi_sticks_in_pot = Some(value);
    }

    pub fn get_riichi_sticks(&self) -> u8 {
        match self.riichi_sticks_in_pot {
            None => 0,
            Some(value) => value,
        }
    }

    pub fn set_tsumibo(&mut self, value: u8) {
        self.tsumibo = Some(value);
    }

    pub fn get_tsumibo(&self) -> u8 {
        match self.tsumibo {
            None => 0,
            Some(value) => value,
        }
    }

    pub fn add_tile_to_discards(&mut self, player: u8, tile: Tile) {
        match player {
            0 => self.my_discards.push(tile),
            1 => self.p1_discards.push(tile),
            2 => self.p2_discards.push(tile),
            3 => self.p3_discards.push(tile),
            _ => panic!("Invalid player")
        }
    }

    /// Finds yaku based on the table state. Some yaku depend on winds, tsumo / ron, tiles remaining etc.
    pub fn yaku(&mut self) -> Option<(Vec<Yaku>, Score)> {
        let yf = YakuFinder::new();
        yf.find(self)
    }

    pub fn can_ankan(&mut self) -> Option<Vec<Tile>> {
        let mut hand = self.get_my_hand().clone();
        if !hand.is_closed() {
            return None;
        }

        if hand.count_tiles() < 14 {
            return None;
        }

        let array_34 = hand.get_34_array(true);
        let mut kannable_tiles = vec![];
        for (i, count) in array_34.iter().enumerate() {
            if *count == 4 {
                kannable_tiles.push((i + 1) as u8);
            }
        }

        if kannable_tiles.is_empty() {
            return None;
        }

        return if self.did_i_riichi() {
            // I can only kan with tiles that won't change my hand structure
            // I can also only kan with the drawn tile
            let mut drawn_tile = hand.get_drawn_tile().unwrap().clone();
            let drawn_tile_id = drawn_tile.to_id();

            if !kannable_tiles.contains(&drawn_tile_id) {
                return None;
            }

            hand.remove_tile_by_id(drawn_tile_id);
            let no_kan_improving_tiles = hand.find_shanten_improving_tiles(None);

            drawn_tile.is_draw = false;
            hand.add_tile(drawn_tile);
            hand.ankan_tiles(drawn_tile);

            hand.reset_shanten();
            let kan_improving_tiles = hand.find_shanten_improving_tiles(None);

            if no_kan_improving_tiles.is_empty() || kan_improving_tiles.is_empty() {
                return None;
            } else {
                let (_to, no_kan_imp_tiles, _ukeire) = no_kan_improving_tiles.get(0).unwrap();
                let (_to, kan_imp_tiles, _ukeire) = kan_improving_tiles.get(0).unwrap();

                if no_kan_imp_tiles.eq(kan_imp_tiles) {
                    return Some(vec![drawn_tile]);
                }
            }

            None
        } else {
            let mut kannable_vec = vec![];
            for tile_id in kannable_tiles.iter() {
                kannable_vec.push(Tile::from_id(*tile_id).unwrap());
            }

            Some(kannable_vec)
        }
    }

    /// How safe is this tile to discard based on this table state?
    /// This function should look at these criteria:
    /// - other players discards
    /// - temporary furiten
    /// - suji
    /// - kabe
    /// - player's tenpai potential (riichi = 100% tenpai...)
    pub fn tile_safety(&self, tile: &Tile) -> u8 {


        0
    }

    /// Guesses a player's tenpai probability based on:
    /// - their riichi state (100% tenpai if riichi)
    /// - discards
    /// - tiles remaining
    /// - calls
    fn tenpai_probability(&self, player: u8) -> u8 {
        let (open_shapes, discards, riichi) = match player {
            1 => (&self.p2_open_tiles, &self.p1_discards, self.p1_riichi.unwrap()),
            2 => (&self.p2_open_tiles, &self.p2_discards, self.p2_riichi.unwrap()),
            3 => (&self.p2_open_tiles, &self.p3_discards, self.p3_riichi.unwrap()),
            _ => panic!("Wrong player ID"),
        };

        0
    }
}

mod tests {
    use super::*;

    #[test]
    fn ankan_test() {
        let mut table = Table {
            my_hand: Some(Hand::from_text("11123m234p456s44z1m", false).unwrap()),
            my_discards: vec![],
            my_riichi: None,
            my_tsumo: None,
            my_points: None,
            p1_discards: vec![],
            p1_open_tiles: vec![],
            p1_riichi: None,
            p1_tsumo: None,
            p1_points: None,
            p2_discards: vec![],
            p2_open_tiles: vec![],
            p2_riichi: None,
            p2_tsumo: None,
            p2_points: None,
            p3_discards: vec![],
            p3_open_tiles: vec![],
            p3_riichi: None,
            p3_tsumo: None,
            p3_points: None,
            prevalent_wind: None,
            my_seat_wind: None,
            wind_round: None,
            total_round: None,
            tiles_remaining: None,
            riichi_sticks_in_pot: None,
            tsumibo: None,
            dora_indicators: vec![],
            visible_tiles: vec![],
            rules: None
        };

        assert!(table.can_ankan() != None);
    }

    #[test]
    fn bad_ankan_riichi_test() {
        let mut table = Table {
            my_hand: Some(Hand::from_text("11123m234p456s44z1m", false).unwrap()),
            my_discards: vec![],
            my_riichi: Some(true),
            my_tsumo: None,
            my_points: None,
            p1_discards: vec![],
            p1_open_tiles: vec![],
            p1_riichi: None,
            p1_tsumo: None,
            p1_points: None,
            p2_discards: vec![],
            p2_open_tiles: vec![],
            p2_riichi: None,
            p2_tsumo: None,
            p2_points: None,
            p3_discards: vec![],
            p3_open_tiles: vec![],
            p3_riichi: None,
            p3_tsumo: None,
            p3_points: None,
            prevalent_wind: None,
            my_seat_wind: None,
            wind_round: None,
            total_round: None,
            tiles_remaining: None,
            riichi_sticks_in_pot: None,
            tsumibo: None,
            dora_indicators: vec![],
            visible_tiles: vec![],
            rules: None
        };

        assert!(table.can_ankan() == None);
    }

    #[test]
    fn good_ankan_riichi_test() {
        let mut table = Table {
            my_hand: Some(Hand::from_text("23666m234p456s44z6m", false).unwrap()),
            my_discards: vec![],
            my_riichi: Some(true),
            my_tsumo: None,
            my_points: None,
            p1_discards: vec![],
            p1_open_tiles: vec![],
            p1_riichi: None,
            p1_tsumo: None,
            p1_points: None,
            p2_discards: vec![],
            p2_open_tiles: vec![],
            p2_riichi: None,
            p2_tsumo: None,
            p2_points: None,
            p3_discards: vec![],
            p3_open_tiles: vec![],
            p3_riichi: None,
            p3_tsumo: None,
            p3_points: None,
            prevalent_wind: None,
            my_seat_wind: None,
            wind_round: None,
            total_round: None,
            tiles_remaining: None,
            riichi_sticks_in_pot: None,
            tsumibo: None,
            dora_indicators: vec![],
            visible_tiles: vec![],
            rules: None
        };

        assert!(table.can_ankan() != None);
    }
}
