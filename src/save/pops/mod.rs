
use std::collections::HashMap;

use super::*;

mod pop;
use pop::Pop;

pub struct Pops {
    database: HashMap<usize, Option<Pop>>
}

impl Pops {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Pop::new_group(value.read_object()?)?),
                "ig_update_index" => {},
                "popgrowth_update_index" => {},
                "popdiscrimination_update_index" => {},
                "poployalties_update_index" => {},
                "pop_weight_randomizer" => {},
                "dead_objects" => {},
                "wealth_initialized" => {},
                a => println!("\t\t\t\t\"{a}\" => {{}},")
            }
        }
        Ok(Self { database: database.unwrap() })
    }
}
