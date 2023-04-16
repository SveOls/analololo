
use std::collections::HashMap;

use super::*;

mod country_formation;
use country_formation::CountryFormation;

pub struct CountryFormations {
    formations: Vec<CountryFormation>,
}

impl CountryFormations {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        let mut formations = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "country_formation_data" => formations = Some(CountryFormation::new_group(value.read_array()?)?),
                a => println!("\t\t\t\t\"{a}\" => !!{{}},")
            }
        }
        Ok(Self { formations: formations.unwrap() })
    }
}
