mod riichi;
extern crate wasm_bindgen;
extern crate serde;
extern crate serde_json;

use wasm_bindgen::prelude::*;
use riichi::hand::Hand;
use riichi::south_4_simulator::South4Simulator;
use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_hand_shanten(hand_string: &str) -> String {
    return match Hand::from_text(hand_string, false) {
        Ok(mut hand) => {
            let shanten = hand.shanten();
            json!({
                "success": {
                    "shanten": shanten
                }
            }).to_string()
        },
        Err(error) => {
            json!({
                "error": {
                    "code": error.code,
                    "message": error.message
                }
            }).to_string()
        }
    }
}

/// Checks the validity of the hand and returns tiles that it found
#[wasm_bindgen]
pub fn get_hand_tiles(hand_string: &str) -> String {
    return match Hand::from_text(hand_string, true) {
        Ok(mut hand) => {
            let valid = hand.validate();
            json!({
                "hand": {
                    "valid": valid,
                    "tiles": hand.to_array_of_strings()
                }
            }).to_string()
        },
        Err(error) => {
            json!({
                "error": {
                    "code": error.code,
                    "message": error.message
                }
            }).to_string()
        }
    }
}

#[wasm_bindgen]
pub fn s4s_start_game() -> String {
    console_error_panic_hook::set_once();

    let simulator = South4Simulator::new();

    json!({
        "game": {
            "my_score": simulator.my_score,
            "opponent_score": simulator.opponent_score,
            "oya_state": simulator.oya_state,
            "riichi_sticks": simulator.riichi_sticks,
            "tsumibo": simulator.tsumibo,
        }
    }).to_string()
}

#[wasm_bindgen]
pub fn s4s_evaluate(my_score: u32,
                    opponent_score: u32,
                    oya_state: u8,
                    riichi_sticks: u8,
                    tsumibo: u8,
                    direct_ron_han: u8,
                    direct_ron_fu: u8,
                    other_ron_han: u8,
                    other_ron_fu: u8,
                    tsumo_han: u8,
                    tsumo_fu: u8,
) -> String {
    let simulator = South4Simulator {
        my_score,
        opponent_score,
        oya_state,
        riichi_sticks,
        tsumibo,
    };

    let result = simulator.evaluate((direct_ron_han, direct_ron_fu), (other_ron_han, other_ron_fu), (tsumo_han, tsumo_fu));

    let result_direct_ron = {result.0}.0;
    let result_other_ron = {result.0}.1;
    let result_tsumo = {result.0}.2;

    let hand_direct_ron = {result.1}.0;
    let hand_other_ron = {result.1}.1;
    let hand_tsumo = {result.1}.2;

    json!({
        "evaluation": {
            "results": {
                "direct_ron": result_direct_ron,
                "other_ron": result_other_ron,
                "tsumo": result_tsumo,
            },
            "correct_hands": {
                "direct_ron": hand_direct_ron,
                "other_ron": hand_other_ron,
                "tsumo": hand_tsumo,
            }
        }
    }).to_string()
}

#[cfg(test)]
mod tests {
    use crate::s4s_start_game;

    #[test]
    fn s4s_start() {
        let json = s4s_start_game();

        println!("{}", json);
    }
}
