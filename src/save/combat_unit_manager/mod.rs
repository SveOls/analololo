use std::collections::HashMap;

use super::*;

mod combat_unit;
use combat_unit::CombatUnit;

pub struct CombatUnitManager {
    database: HashMap<usize, Option<CombatUnit>>,
}

impl CombatUnitManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "dead" => {}
                "hq_allocated_units" => {}
                "database" => database = Some(CombatUnit::new_group(value.read_object()?)?),
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
