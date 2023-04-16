use std::collections::HashMap;

use super::*;

mod proposal;
use proposal::Proposal;

#[allow(dead_code)]
pub struct Proposals {
    database: HashMap<usize, Option<Proposal>>,
}

impl Proposals {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Proposal::new_group(value.read_object()?)?),
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
