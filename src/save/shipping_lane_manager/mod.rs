use std::collections::HashMap;

use super::*;

mod shipping_lane;
use shipping_lane::ShippingLane;

pub struct ShippingLaneManager {
    database: HashMap<usize, Option<ShippingLane>>,
}

impl ShippingLaneManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(ShippingLane::new_group(value.read_object()?)?),
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
