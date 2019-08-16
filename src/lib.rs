mod riichi;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use riichi::hand::Hand;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn get_hand_shanten(hand_string: &str) -> u8 {
    let mut hand = Hand::from_text(hand_string);
    let shanten = hand.shanten();

    shanten
}