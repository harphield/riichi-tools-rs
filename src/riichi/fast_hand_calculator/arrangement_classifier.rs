use crate::riichi::fast_hand_calculator::resources::RESOURCES;

pub struct ArrangementClassifier {}

impl ArrangementClassifier {
    pub fn new() -> Self {
        Self {}
    }

    pub fn classify(&self, values: &[u32; 4]) -> u32 {
        let transitions = RESOURCES.get_arrangement_transitions();
        let mut current = transitions[values[0] as usize];
        current = transitions[(current + values[1]) as usize];
        current = transitions[(current + values[2]) as usize];
        current = transitions[(current + values[3]) as usize];

        current
    }
}
