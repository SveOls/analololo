use std::collections::HashMap;

use super::*;

mod naval_invasion;
use naval_invasion::NavalInvasion;

pub struct NavalInvasions {
    database: HashMap<usize, Option<NavalInvasion>>,
}

impl NavalInvasions {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(NavalInvasion::new_group(value.read_object()?)?),
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
