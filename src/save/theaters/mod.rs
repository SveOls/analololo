use std::collections::HashMap;

use super::*;

mod theater;
use theater::Theater;

#[allow(dead_code)]
pub struct Theaters {
    database: HashMap<usize, Option<Theater>>,
}

impl Theaters {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Theater::new_group(value.read_object()?)?),
                "theater_countries" => {}
                "dead" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
