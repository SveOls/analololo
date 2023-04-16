use std::collections::HashMap;

use super::*;

mod order;
use order::Order;

#[allow(dead_code)]
pub struct OrderManager {
    database: HashMap<usize, Option<Order>>,
}

impl OrderManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Order::new_group(value.read_object()?)?),
                "dead" => {}
                "interception_data" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
