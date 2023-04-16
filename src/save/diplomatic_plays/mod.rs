use std::collections::HashMap;

use super::*;

mod diplomatic_play;
use diplomatic_play::DiplomaticPlay;

#[allow(dead_code)]
pub struct DiplomaticPlays {
    database: HashMap<usize, Option<DiplomaticPlay>>,
}

impl DiplomaticPlays {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(DiplomaticPlay::new_group(value.read_object()?)?),
                "dead" => {}
                "diplomatic_play_associated_civil_wars" => {}
                "active_diplomatic_plays" => {}
                "escalating_diplomatic_plays" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
