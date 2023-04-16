
use std::collections::HashMap;

use super::*;

mod country;
use country::Country;

pub struct CountryManager {
    database: HashMap<usize, Option<Country>>
}

impl CountryManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Country::new_group(value.read_object()?)?),
                "dead_countries" => {},
                "construction_points_counter" => {},
                a => println!("\t\t\t\t\"{a}\" => {{}},")
            }
        }
        Ok(Self { database: database.unwrap() })
    }
}
