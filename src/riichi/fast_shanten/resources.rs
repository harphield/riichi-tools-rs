use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "res/"]
pub struct Resources;

impl Resources {
    fn get_suit_first_phase() -> Vec<u32> {
        let x = Resources::get("SuitFirstPhase.txt").unwrap();

        vec![]
    }

    pub fn prepare_data_from_string(data: &str) -> Vec<u32> {
        let x: Vec<u32> = data.split('\n').map(|line| {
            let num = line.parse::<i32>().unwrap();

            if num < 0 {
                return 0;
            }

            num as u32
        }).collect();

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
    }
}