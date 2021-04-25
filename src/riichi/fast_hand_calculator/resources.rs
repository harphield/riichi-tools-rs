use rust_embed::RustEmbed;

lazy_static! {
    pub static ref RESOURCES: Resources = Resources::new();
}

#[derive(RustEmbed)]
#[folder = "res/"]
struct Asset;

pub struct Resources {
    suit_first_phase: Vec<u32>,
    suit_second_phase_0: Vec<u32>,
    suit_second_phase_1: Vec<u32>,
    suit_second_phase_2: Vec<u32>,
    suit_second_phase_3: Vec<u32>,
    suit_second_phase_4: Vec<u32>,
    suit_base_5_lookup: Vec<u8>,
    arrangement_transitions: Vec<u32>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            suit_first_phase: Resources::init_suit_first_phase(),
            suit_second_phase_0: Resources::init_suit_second_phase(0),
            suit_second_phase_1: Resources::init_suit_second_phase(1),
            suit_second_phase_2: Resources::init_suit_second_phase(2),
            suit_second_phase_3: Resources::init_suit_second_phase(3),
            suit_second_phase_4: Resources::init_suit_second_phase(4),
            suit_base_5_lookup: Resources::init_suit_base_5_lookup(),
            arrangement_transitions: Resources::init_arrangement_transitions(),
        }
    }

    pub fn get_suit_first_phase(&self) -> &Vec<u32> {
        &self.suit_first_phase
    }

    pub fn get_suit_second_phase(&self, i: u8) -> Option<&Vec<u32>> {
        match i {
            0 => Some(&self.suit_second_phase_0),
            1 => Some(&self.suit_second_phase_1),
            2 => Some(&self.suit_second_phase_2),
            3 => Some(&self.suit_second_phase_3),
            4 => Some(&self.suit_second_phase_4),
            _ => None,
        }
    }

    pub fn get_suit_base_5_lookup(&self) -> &Vec<u8> {
        &self.suit_base_5_lookup
    }

    pub fn get_arrangement_transitions(&self) -> &Vec<u32> {
        &self.arrangement_transitions
    }

    fn init_suit_first_phase() -> Vec<u32> {
        let file = Asset::get("SuitFirstPhase.txt").unwrap();
        Resources::prepare_data_from_string(std::str::from_utf8(file.as_ref()).unwrap())
    }

    fn init_suit_second_phase(i: u8) -> Vec<u32> {
        let file = Asset::get(&format!("SuitSecondPhase{}.txt", i)[..]).unwrap();
        Resources::prepare_data_from_string(std::str::from_utf8(file.as_ref()).unwrap())
    }

    fn init_suit_base_5_lookup() -> Vec<u8> {
        Asset::get("suitArrangementsBase5NoMelds.dat")
            .unwrap()
            .to_vec()
    }

    fn init_arrangement_transitions() -> Vec<u32> {
        let file = Asset::get("ArrangementTransitions.txt").unwrap();
        Resources::prepare_data_from_string(std::str::from_utf8(file.as_ref()).unwrap())
    }

    fn prepare_data_from_string(data: &str) -> Vec<u32> {
        let x: Vec<u32> = data
            .split('\n')
            .map(|line| match line.parse::<i32>() {
                Ok(num) => {
                    if num < 0 {
                        return 0;
                    }

                    num as u32
                }
                Err(_) => 0,
            })
            .collect();

        x
    }
}

#[cfg(test)]
mod tests {
    use crate::riichi::fast_hand_calculator::resources::Resources;

    #[test]
    fn load_first_phase() {
        let res = Resources::new();
        let fp = res.get_suit_first_phase();
        assert_ne!(fp.len(), 0);
        assert_eq!(fp.get(1).unwrap(), &26);
    }

    #[test]
    fn load_second_phase() {
        let res = Resources::new();
        let fp = res.get_suit_second_phase(2).unwrap();
        assert_ne!(fp.len(), 0);
        assert_eq!(fp.get(0).unwrap(), &5580);
    }

    #[test]
    fn load_base_5() {
        let res = Resources::new();
        let fp = res.get_suit_base_5_lookup();
        assert_ne!(fp.len(), 0);
        assert_eq!(fp.get(2).unwrap(), &3);
    }
}
