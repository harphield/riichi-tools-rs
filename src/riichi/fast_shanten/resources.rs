use rust_embed::RustEmbed;
use std::num::ParseIntError;

#[derive(RustEmbed)]
#[folder = "res/"]
pub struct Resources;

impl Resources {
    fn get_suit_first_phase() -> Vec<u32> {
        let file = Resources::get("SuitFirstPhase.txt").unwrap();
        Resources::prepare_data_from_string(std::str::from_utf8(file.as_ref()).unwrap())
    }

    pub fn prepare_data_from_string(data: &str) -> Vec<u32> {
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
    use crate::riichi::fast_shanten::resources::Resources;

    #[test]
    fn load_first_phase() {
        let fp = Resources::get_suit_first_phase();
        assert_ne!(fp.len(), 0);
        assert_eq!(fp.get(1).unwrap(), &26);
    }
}
