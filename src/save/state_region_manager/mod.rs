
use std::collections::HashMap;

use super::*;

mod state_region;
use state_region::StateRegion;

pub struct StateRegionManager {
    database: HashMap<usize, Option<StateRegion>>,
}

impl StateRegionManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(StateRegion::new_group(value.read_object()?)?),
                a => println!("\t\t\t\t\"{a}\" => !!{{}},")
            }
        }
        Ok(Self { database: database.unwrap() })
    }
}
