use std::collections::HashMap;

use super::*;

mod building;
use building::Building;

pub struct BuildingManager {
    database: HashMap<usize, Option<Building>>,
}

impl BuildingManager {
    pub fn database(&self) -> &HashMap<usize, Option<Building>> {
        &self.database
    }
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Building::new_group(value.read_object()?)?),
                "dead_buildings" => {}
                "average_employee_earnings" => {}
                "building_subsidies_counters" => {}
                "building_max_level_counters" => {}
                "building_production_method_counters" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
