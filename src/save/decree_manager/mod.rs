use std::collections::HashMap;

use super::*;

mod decree;
use decree::Decree;

pub struct DecreeManager {
    database: HashMap<usize, Option<Decree>>,
}

impl DecreeManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Decree::new_group(value.read_object()?)?),
                "dead_objects" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
