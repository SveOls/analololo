
use std::collections::HashMap;

use super::*;

mod market;
use market::Market;

pub struct MarketManager {
    database: HashMap<usize, Option<Market>>
}

impl MarketManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(Market::new_group(value.read_object()?)?),
                "markets_counter" => {},
                a => println!("\t\t\t\t\"{a}\" => {{}},")
            }
        }
        Ok(Self { database: database.unwrap() })
    }
}
