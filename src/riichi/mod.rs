#[cfg(feature = "fast_shanten")]
mod fast_hand_calculator;
pub mod hand;
pub mod potential;
pub mod riichi_error;
pub mod rules;
pub mod scores;
#[cfg(not(feature = "fast_shanten"))]
mod shanten;
mod shape_finder;
pub mod shapes;
pub mod table;
pub mod tile;
pub mod yaku;
