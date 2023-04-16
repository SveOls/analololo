use std::collections::HashMap;

use super::*;

mod relation;
use relation::Relation;

#[allow(dead_code)]
pub struct Relations {
    database: HashMap<usize, Option<Relation>>,
}

impl Relations {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Relation::new_group(value.read_object()?)?),
                "dead" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
