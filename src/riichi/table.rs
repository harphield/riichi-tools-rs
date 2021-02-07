use crate::riichi::hand::Hand;
use crate::riichi::riichi_error::RiichiError;
use crate::riichi::rules::Rules;
use crate::riichi::scores::Score;
use crate::riichi::shapes::Shape;
use crate::riichi::tile::Tile;
use crate::riichi::yaku::{Yaku, YakuFinder};
use serde_json::{Map, Value};

/// Representation of the game state
pub struct Table {
    my_hand: Option<Hand>,
    my_discards: Vec<Tile>,
    my_riichi: Option<bool>,
    my_tsumo: Option<bool>,
    my_points: Option<i32>,
    // player to the right (shimocha)
    p1_discards: Vec<Tile>,
    p1_safe_tiles: Vec<Tile>,
    p1_open_tiles: Vec<Shape>,
    p1_riichi: Option<bool>,
    p1_tsumo: Option<bool>,
    p1_points: Option<i32>,
    // opposite player (toimen)
    p2_discards: Vec<Tile>,
    p2_safe_tiles: Vec<Tile>,
    p2_open_tiles: Vec<Shape>,
    p2_riichi: Option<bool>,
    p2_tsumo: Option<bool>,
    p2_points: Option<i32>,
    // player to the left (kamicha)
    p3_discards: Vec<Tile>,
    p3_safe_tiles: Vec<Tile>,
    p3_open_tiles: Vec<Shape>,
    p3_riichi: Option<bool>,
    p3_tsumo: Option<bool>,
    p3_points: Option<i32>,

    riichi_declaring_player: Option<u8>,

    // 1 = east, 2 = south, 3 = west, 4 = north
    prevalent_wind: Option<u8>,
    my_seat_wind: Option<u8>,
    my_initial_seat_wind: Option<u8>,
    /// E1 - E4, S1-S4...
    dealer_turn: Option<u8>,
    total_round: Option<u8>,
    tiles_remaining: Option<u8>,

    riichi_sticks_in_pot: Option<u8>,
    tsumibo: Option<u8>,

    dora_indicators: Vec<Tile>,
    visible_tiles: [u8; 34], // in array_34 format

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
            p1_safe_tiles: vec![],
            p1_open_tiles: vec![],
            p1_riichi: None,
            p1_tsumo: None,
            p1_points: None,
            p2_discards: vec![],
            p2_safe_tiles: vec![],
            p2_open_tiles: vec![],
            p2_riichi: None,
            p2_tsumo: None,
            p2_points: None,
            p3_discards: vec![],
            p3_safe_tiles: vec![],
            p3_open_tiles: vec![],
            p3_riichi: None,
            p3_tsumo: None,
            p3_points: None,
            riichi_declaring_player: None,
            prevalent_wind: None,
            my_seat_wind: None,
            my_initial_seat_wind: None,
            dealer_turn: None,
            total_round: None,
            tiles_remaining: None,
            riichi_sticks_in_pot: None,
            tsumibo: None,
            dora_indicators: vec![],
            visible_tiles: [0; 34],
            rules: None,
        };

        for (index, value) in params {
            if index.eq(&String::from("my_hand")) {
                if let Value::String(s) = value {
                    match Hand::from_text(s, false) {
                        Ok(hand) => t.my_hand = Some(hand),
                        Err(error) => return Err(error),
                    }
                }
            } else if index.eq(&String::from("my_riichi")) {
                if let Value::Bool(b) = value {
                    t.my_riichi = Some(*b);
                }
            } else if index.eq(&String::from("my_tsumo")) {
                if let Value::Bool(b) = value {
                    t.my_tsumo = Some(*b);
                }
            } else if index.eq(&String::from("prevalent_wind")) {
                match value {
                    Value::Number(v) => {
                        t.prevalent_wind = Some(v.as_u64().unwrap() as u8);
                    }
                    Value::String(v) => {
                        let number: u8 = v.parse().unwrap();
                        t.prevalent_wind = Some(number);
                    }
                    _ => (),
                }
            } else if index.eq(&String::from("my_seat_wind")) {
                match value {
                    Value::Number(v) => {
                        t.my_seat_wind = Some(v.as_u64().unwrap() as u8);
                    }
                    Value::String(v) => {
                        let number: u8 = v.parse().unwrap();
                        t.my_seat_wind = Some(number);
                    }
                    _ => (),
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
            Some(seat) => seat == 1,
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
        self.my_tsumo.unwrap_or(false)
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

    pub fn get_p1_riichi(&self) -> bool {
        self.p1_riichi.unwrap_or(false)
    }

    pub fn get_p2_riichi(&self) -> bool {
        self.p2_riichi.unwrap_or(false)
    }

    pub fn get_p3_riichi(&self) -> bool {
        self.p3_riichi.unwrap_or(false)
    }

    pub fn set_riichi_declaring_player(&mut self, player_id: u8) {
        self.riichi_declaring_player = Some(player_id);

        if player_id == 1 {
            self.set_p1_riichi(true);
        } else if player_id == 2 {
            self.set_p2_riichi(true);
        } else if player_id == 3 {
            self.set_p3_riichi(true);
        }
    }

    pub fn unset_riichi_declaring_player(&mut self) {
        self.riichi_declaring_player = None;
    }

    pub fn get_riichi_declaring_player(&self) -> Option<u8> {
        self.riichi_declaring_player
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

    pub fn get_my_hand(&self) -> &Hand {
        match &self.my_hand {
            None => panic!("No hand!"),
            Some(hand) => &hand,
        }
    }

    pub fn get_my_hand_option(&self) -> &Option<Hand> { &self.my_hand }

    pub fn get_my_winning_tile(&self) -> Tile {
        match &self.my_hand {
            None => panic!("No drawn tile in hand!"),
            Some(hand) => *hand.get_drawn_tile().unwrap(),
        }
    }

    pub fn set_tiles_remaining(&mut self, value: u8) {
        self.tiles_remaining = Some(value);
    }

    pub fn get_tiles_remaining(&self) -> Option<u8> {
        self.tiles_remaining
    }

    pub fn decrement_tiles_remaining(&mut self) {
        match self.tiles_remaining {
            None => {
                self.set_tiles_remaining(0);
            }
            Some(value) => {
                self.set_tiles_remaining(value - 1);
            }
        }
    }

    pub fn set_my_seat_wind(&mut self, value: u8) {
        self.my_seat_wind = Some(value);
    }

    pub fn get_my_seat_wind(&self) -> Option<u8> {
        self.my_seat_wind
    }

    pub fn set_my_initial_seat_wind(&mut self, value: u8) {
        self.my_initial_seat_wind = Some(value);
    }

    pub fn get_my_initial_seat_wind(&self) -> Option<u8> {
        self.my_initial_seat_wind
    }

    pub fn set_prevalent_wind(&mut self, value: u8) {
        self.prevalent_wind = Some(value);
    }

    pub fn get_prevalent_wind(&self) -> Option<u8> {
        self.prevalent_wind
    }

    pub fn set_dealer_turn(&mut self, value: u8) {
        self.dealer_turn = Some(value);
    }

    pub fn get_dealer_turn(&self) -> Option<u8> { self.dealer_turn }

    pub fn get_my_points(&self) -> Option<i32> { self.my_points }

    pub fn get_placing(&self) -> u8 {
        // TODO: account for different tie-breaking rules
        let initial_seat_winds = match self.my_initial_seat_wind {
            Some(1) => vec![1, 2, 3, 4],
            Some(2) => vec![2, 3, 4, 1],
            Some(3) => vec![3, 4, 1, 2],
            _ => vec![4, 1, 2, 3],
        };
        let points = vec![
            self.my_points.unwrap_or(25000),
            self.p1_points.unwrap_or(25000),
            self.p2_points.unwrap_or(25000),
            self.p3_points.unwrap_or(25000),
        ];
        let mut all_scores: Vec<_> = (points.iter()).zip(initial_seat_winds.iter()).collect();
        // sort from lowest to hightest
        all_scores.sort_by(|a, b| b.0.cmp(a.0).then(a.1.cmp(b.1)));

        all_scores
            .iter()
            .position(|score| *score.1 == self.my_initial_seat_wind.unwrap_or(4))
            .unwrap_or(4) as u8
            + 1
    }

    pub fn set_dora_indicators(&mut self, indicators: Vec<Tile>) {
        self.dora_indicators = indicators;
    }

    pub fn add_dora_indicator(&mut self, indicator: Tile) {
        self.dora_indicators.push(indicator);
        self.visible_tiles[(indicator.to_id() - 1) as usize] += 1;
    }

    pub fn get_dora_indicators(&self) -> &Vec<Tile> {
        &self.dora_indicators
    }

    pub fn add_tile_to_visible_tiles(&mut self, tile: Tile) {
        self.visible_tiles[(tile.to_id() - 1) as usize] += 1;
    }

    pub fn reset_tile_vectors(&mut self) {
        self.visible_tiles = [0; 34];
        self.my_discards = vec![];
        self.p1_discards = vec![];
        self.p2_discards = vec![];
        self.p3_discards = vec![];
        self.p1_safe_tiles = vec![];
        self.p2_safe_tiles = vec![];
        self.p3_safe_tiles = vec![];
    }

    pub fn get_visible_tiles(&self) -> &[u8; 34] {
        &self.visible_tiles
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
        self.riichi_sticks_in_pot.unwrap_or(0)
    }

    pub fn set_tsumibo(&mut self, value: u8) {
        self.tsumibo = Some(value);
    }

    pub fn get_tsumibo(&self) -> u8 {
        self.tsumibo.unwrap_or(0)
    }

    pub fn set_rules(&mut self, rules: Rules) {
        self.rules = Some(rules);
    }

    pub fn get_rules(&self) -> &Option<Rules> {
        &self.rules
    }

    pub fn get_my_discards(&self) -> &Vec<Tile> { &self.my_discards }

    pub fn add_tile_to_discards(&mut self, player: u8, tile: Tile) {
        match player {
            0 => self.my_discards.push(tile),
            1 => self.p1_discards.push(tile),
            2 => self.p2_discards.push(tile),
            3 => self.p3_discards.push(tile),
            _ => panic!("Invalid player"),
        }
    }

    pub fn add_tile_to_safe_tiles(&mut self, player: u8, tile: Tile) {
        match player {
            1 => self.p1_safe_tiles.push(tile),
            2 => self.p2_safe_tiles.push(tile),
            3 => self.p3_safe_tiles.push(tile),
            _ => panic!("Invalid player"),
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
            let mut drawn_tile = *hand.get_drawn_tile().unwrap();
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
                let (_to, no_kan_ukeire, _total_count) = no_kan_improving_tiles.get(0).unwrap();
                let (_to, kan_ukeire, _total_count) = kan_improving_tiles.get(0).unwrap();

                if no_kan_ukeire.eq(kan_ukeire) {
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
        };
    }

    /// How safe is this tile to discard based on this table state?
    /// This function should look at these criteria:
    /// - other players discards
    /// - temporary furiten
    /// - other player's calls (chinitsu / honitsu / sanshoku / ittsu calls etc.)
    /// - suji
    /// - kabe
    /// - player's tenpai potential (riichi = 100% tenpai...)
    pub fn tile_safety(&self, tile: &Tile) -> f32 {
        let tenpai_probs = [
            self.tenpai_probability(1),
            self.tenpai_probability(2),
            self.tenpai_probability(3),
        ];

        let discards = [&self.p1_discards, &self.p2_discards, &self.p3_discards];

        let last_discards = [discards[0].last(), discards[1].last(), discards[2].last()];

        let safe_tiles = [
            &self.p1_safe_tiles,
            &self.p2_safe_tiles,
            &self.p3_safe_tiles,
        ];

        // let open_tiles = [
        //     &self.p1_open_tiles,
        //     &self.p2_open_tiles,
        //     &self.p3_open_tiles,
        // ];

        // percentage of safety against players
        let mut safeties = [
            0.0, // p1 (kamicha)
            0.0, // p2 (toimen)
            0.0, // p3 (shimocha)
        ];

        if discards[0].contains(&tile)
            || safe_tiles[0].contains(&tile)
            || self.is_temporary_furiten(&tile, vec![&last_discards[1], &last_discards[2]])
        {
            safeties[0] = 1.0;
        } else {
        }

        if discards[1].contains(&tile)
            || safe_tiles[1].contains(&tile)
            || self.is_temporary_furiten(&tile, vec![&last_discards[0], &last_discards[2]])
        {
            safeties[1] = 1.0;
        } else {
        }

        if discards[2].contains(&tile)
            || safe_tiles[2].contains(&tile)
            || self.is_temporary_furiten(&tile, vec![&last_discards[0], &last_discards[1]])
        {
            safeties[2] = 1.0;
        } else {
        }

        // weigh safeties by tenpai probability
        safeties[0] *= tenpai_probs[0];
        safeties[1] *= tenpai_probs[1];
        safeties[2] *= tenpai_probs[2];

        safeties.iter().sum::<f32>() / safeties.len() as f32
    }

    fn is_temporary_furiten(&self, tile: &Tile, last_discards: Vec<&Option<&Tile>>) -> bool {
        for ld_o in last_discards.iter() {
            match ld_o {
                None => {}
                Some(ld) => {
                    if ld.eq(&tile) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Guesses a player's tenpai probability based on:
    /// - their riichi state (100% tenpai if riichi)
    /// - discards
    /// - tiles remaining
    /// - calls
    pub fn tenpai_probability(&self, player: u8) -> f32 {
        let (open_shapes, _discards, riichi) = match player {
            1 => (
                &self.p1_open_tiles,
                &self.p1_discards,
                self.p1_riichi.unwrap(),
            ),
            2 => (
                &self.p2_open_tiles,
                &self.p2_discards,
                self.p2_riichi.unwrap(),
            ),
            3 => (
                &self.p3_open_tiles,
                &self.p3_discards,
                self.p3_riichi.unwrap(),
            ),
            _ => panic!("Wrong player ID"),
        };

        if riichi {
            return 1.0;
        }

        // 4 open shapes = tanki wait
        if open_shapes.len() == 4 {
            return 1.0;
        }

        // TODO more stuff

        0.0
    }
}

mod tests {

    #[test]
    fn placing_test() {
        use super::*;
        let mut table = Table::from_map(&Map::new()).unwrap();
        // table.set_my_hand(Hand::from_text("11123m234p456s44z1m", false).unwrap());

        table.my_initial_seat_wind = Some(3);
        assert!(table.get_placing() == 3);
        table.my_points = Some(20000);
        assert!(table.get_placing() == 4);
    }

    #[test]
    fn ankan_test() {
        use super::*;
        let mut table = Table::from_map(&Map::new()).unwrap();
        table.set_my_hand(Hand::from_text("11123m234p456s44z1m", false).unwrap());

        assert!(table.can_ankan() != None);
    }

    #[test]
    fn bad_ankan_riichi_test() {
        use super::*;
        let mut table = Table::from_map(&Map::new()).unwrap();
        table.set_my_hand(Hand::from_text("11123m234p456s44z1m", false).unwrap());
        table.set_my_riichi(true);

        assert!(table.can_ankan() == None);
    }

    #[test]
    fn good_ankan_riichi_test() {
        use super::*;
        let mut table = Table::from_map(&Map::new()).unwrap();
        table.set_my_hand(Hand::from_text("23666m234p456s44z6m", false).unwrap());
        table.set_my_riichi(true);

        assert!(table.can_ankan() != None);
    }

    #[test]
    fn safety_riichi() {
        use super::*;
        let mut table = Table::from_map(&Map::new()).unwrap();
        table.set_my_hand(Hand::from_text("23666m234p456s44z6m", false).unwrap());

        let tile = Tile::from_text("2m").unwrap();

        table.set_p1_riichi(true);
        table.set_p2_riichi(false);
        table.set_p3_riichi(false);

        table.add_tile_to_discards(1, tile);
        table.add_tile_to_safe_tiles(1, tile);

        let safety = table.tile_safety(&tile);

        println!("{}", safety);
    }
}
