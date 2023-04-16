
use std::collections::HashMap;

use super::*;

mod objective;
use objective::Objective;

pub struct ObjectiveManager {
    database: HashMap<usize, Option<Objective>>,
}

impl ObjectiveManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Objective::new_group(value.read_object()?)?),
                "dead" => {},
                "player_objectives" => {},
                a => println!("\t\t\t\t\"{a}\" => !!{{}},")
            }
        }
        Ok(Self { database: database.unwrap() })
    }
}
