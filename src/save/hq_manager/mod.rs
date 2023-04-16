use std::collections::HashMap;

use super::*;

mod hq;
use hq::Hq;

#[allow(dead_code)]
pub struct HqManager {
    database: HashMap<usize, Option<Hq>>,
}

impl HqManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Hq::new_group(value.read_object()?)?),
                "dirty_strategic_regions" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
