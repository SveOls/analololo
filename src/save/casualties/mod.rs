
use std::collections::HashMap;

use super::*;

mod casualty;
use casualty::Casualty;

pub struct Casualties {
    database: HashMap<usize, Option<Casualty>>,
}

impl Casualties {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Casualty::new_group(value.read_object()?)?),
                a => println!("\t\t\t\t\"{a}\" => !!{{}},")
            }
        }
        Ok(Self { database: database.unwrap() })
    }
}
