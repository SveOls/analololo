use std::collections::HashMap;

use super::*;

mod culture;
use culture::Culture;

pub struct Cultures {
    database: HashMap<usize, Option<Culture>>,
}

impl Cultures {
    pub fn database(&self) -> &HashMap<usize, Option<Culture>> {
        &self.database
    }
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Culture::new_group(value.read_object()?)?),
                "dead" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
