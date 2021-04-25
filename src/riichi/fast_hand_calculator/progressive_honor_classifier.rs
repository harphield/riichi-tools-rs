use crate::riichi::fast_hand_calculator::resources::RESOURCES;

/// Returns the arrangement value of honors after the execution of a single action.
/// actionIds:
///
/// 0  draw with 0 of the same type in concealed hand
/// 1  draw with 1
/// 2  draw with 2
/// 3  draw with 3
///
/// 4  draw with pon of the same type
///
/// 5  discard with 1 of the same type in concealed hand
/// 6  discard with 2
/// 7  discard with 3
/// 8  discard with 4
///
/// 9  discard with pon of the same type
///
/// 10 pon with 2 of the same type in hand before pon
/// 11 pon with 3
///
/// 12 daiminkan
///
/// 13 shouminkan
///
/// 14 ankan
///
/// The next state is always at current + 1 + actionId
pub struct ProgressiveHonorClassifier {
    current: u32,
}

impl ProgressiveHonorClassifier {
    pub fn new() -> ProgressiveHonorClassifier {
        ProgressiveHonorClassifier { current: 0 }
    }

    pub fn draw(&mut self, previous_tiles: u8, meld_bit: u8) -> u32 {
        let action = previous_tiles + (meld_bit << 2) + 1;
        self.current = RESOURCES.get_honor_state_machine()[(self.current + action as u32) as usize];
        RESOURCES.get_honor_state_machine()[self.current as usize]
    }

    pub fn discard(&mut self, tiles_after_discard: u8, meld_bit: u8) -> u32 {
        let action = tiles_after_discard + (meld_bit << 2) + 6;
        self.current = RESOURCES.get_honor_state_machine()[(self.current + action as u32) as usize];
        RESOURCES.get_honor_state_machine()[self.current as usize]
    }

    pub fn pon(&mut self, previous_tiles: u8) -> u32 {
        self.current = RESOURCES.get_honor_state_machine()
            [(self.current + previous_tiles as u32 + 9) as usize];
        RESOURCES.get_honor_state_machine()[self.current as usize]
    }

    pub fn daiminkan(&mut self) -> u32 {
        self.current = RESOURCES.get_honor_state_machine()[(self.current + 13) as usize];
        RESOURCES.get_honor_state_machine()[self.current as usize]
    }

    pub fn shouminkan(&mut self) -> u32 {
        self.current = RESOURCES.get_honor_state_machine()[(self.current + 14) as usize];
        RESOURCES.get_honor_state_machine()[self.current as usize]
    }

    pub fn ankan(&mut self) -> u32 {
        self.current = RESOURCES.get_honor_state_machine()[(self.current + 15) as usize];
        RESOURCES.get_honor_state_machine()[self.current as usize]
    }
}
