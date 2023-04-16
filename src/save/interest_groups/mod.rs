use std::collections::HashMap;

use super::*;

mod interest_group;
use interest_group::InterestGroup;

pub struct InterestGroups {
    database: HashMap<usize, Option<InterestGroup>>,
}

impl InterestGroups {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(InterestGroup::new_group(value.read_object()?)?),
                "dead" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
