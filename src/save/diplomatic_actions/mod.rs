
use std::collections::HashMap;

use super::*;

mod diplomatic_actions;
use diplomatic_actions::DiplomaticAction;

pub struct DiplomaticActions {
    database: HashMap<usize, Option<DiplomaticAction>>,
}

impl DiplomaticActions {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(DiplomaticAction::new_group(value.read_object()?)?),
                "dead" => {},
                a => println!("\t\t\t\t\"{a}\" => !!{{}},")
            }
        }
        Ok(Self { database: database.unwrap() })
    }
}
