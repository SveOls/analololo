use std::collections::HashMap;

use super::*;

mod civil_war_one;
use civil_war_one::CivilWarOne;

#[allow(dead_code)]
pub struct CivilWar {
    database: HashMap<usize, Option<CivilWarOne>>,
}

impl CivilWar {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(CivilWarOne::new_group(value.read_object()?)?),
                "dead" => {}
                "cooldown" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
