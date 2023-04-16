use std::collections::HashMap;

use super::*;

mod techs;
use techs::Techs;

#[allow(dead_code)]
pub struct Technology {
    database: HashMap<usize, Option<Techs>>,
}

impl Technology {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Techs::new_group(value.read_object()?)?),
                "dead" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
