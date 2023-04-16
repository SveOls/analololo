
use std::collections::HashMap;

use super::*;

mod war;
use war::War;

pub struct WarManager {
    database: HashMap<usize, Option<War>>,
}

impl WarManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(War::new_group(value.read_object()?)?),
                a => println!("\t\t\t\t\"{a}\" => !!{{}},")
            }
        }
        Ok(Self { database: database.unwrap() })
    }
}
