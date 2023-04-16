use std::collections::HashMap;

use super::*;

mod character;
use character::Character;

pub struct CharacterManager {
    database: HashMap<usize, Option<Character>>,
}

impl CharacterManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Character::new_group(value.read_object()?)?),
                "ig_characters_map" => {}
                "country_character_map" => {}
                "country_character_offer_map" => {}
                "character_ig_map" => {}
                "character_country_map" => {}
                "home_hq_character_map" => {}
                "location_hq_character_map" => {}
                "character_home_hq_map" => {}
                "character_location_hq_map" => {}
                "character_death_queue" => {}
                "busy_characters" => {}
                "dead_objects" => {}
                "used_templates" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}} ,"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
