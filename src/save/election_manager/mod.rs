use std::collections::HashMap;

use super::*;

mod election;
use election::Election;

#[allow(dead_code)]
pub struct ElectionManager {
    database: HashMap<usize, Option<Election>>,
}

impl ElectionManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Election::new_group(value.read_object()?)?),
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
