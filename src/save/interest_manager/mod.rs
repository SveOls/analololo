use std::collections::HashMap;

use super::*;

mod interest;
use interest::Interest;

#[allow(dead_code)]
pub struct InterestManager {
    database: HashMap<usize, Option<Interest>>,
}

impl InterestManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Interest::new_group(value.read_object()?)?),
                "dead_objects" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
