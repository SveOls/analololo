use std::collections::HashMap;

use super::*;

mod law;
use law::Law;

#[allow(dead_code)]
pub struct Laws {
    database: HashMap<usize, Option<Law>>,
}

impl Laws {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Law::new_group(value.read_object()?)?),
                "dead" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
