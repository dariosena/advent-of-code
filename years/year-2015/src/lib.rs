use std::{error::Error, fs};

pub struct InputHelper;

impl InputHelper {
    const BASE_PATH: &str = env!("CARGO_MANIFEST_DIR");
    const RESOURCE_PATH: &str = "/resources";

    pub fn read_input(day: &str, part: &str) -> Result<String, Box<dyn Error>> {
        let mut filename = String::from(Self::BASE_PATH);

        filename.push_str(Self::RESOURCE_PATH);
        filename.push_str(&format!("/{}", day));
        filename.push_str(&format!("/{}.txt", part));

        let input = fs::read_to_string(filename)?;

        Ok(input)
    }
}
