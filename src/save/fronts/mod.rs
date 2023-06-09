use std::collections::HashMap;

use super::*;

mod front;
use front::Front;

#[allow(dead_code)]
pub struct Fronts {
    database: HashMap<usize, Option<Front>>,
}

impl Fronts {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Front::new_group(value.read_object()?)?),
                "theater_counter" => {}
                "diplomatic_play_participants" => {}
                "front_to_generals" => {}
                "split_theaters" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
