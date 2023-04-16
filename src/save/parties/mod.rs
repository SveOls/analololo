
use std::collections::HashMap;

use super::*;

mod party;
mod potential;
use party::Party;
use potential::Potential;

pub struct Parties {
    database: HashMap<usize, Party>,
    countries: Vec<Potential>,
}

impl Parties {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        let mut database = None;
        let mut countries = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "countries" => countries = Some(Potential::new_group(value.read_array()?)?),
                "database" => database = Some(Party::new_group(value.read_object()?)?),
                a => println!("\t\t\t\t\"{a}\" => !!{{}},")
            }
        }
        Ok(Self { database: database.unwrap(), countries: countries.unwrap() })
    }
}
