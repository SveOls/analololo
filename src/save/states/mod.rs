use std::collections::HashMap;

use super::*;

mod state;
use state::State;

pub struct States {
    database: HashMap<usize, Option<State>>,
    statearray: Vec<Vec<usize>>,
}

impl States {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;
        let mut statearray = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(State::new_group(value.read_object()?)?),
                "state_region_to_state_array" => {
                    statearray = Some(
                        value
                            .read_array()?
                            .values()
                            .map(|x| {
                                x.read_array()
                                    .unwrap()
                                    .values()
                                    .map(|x| x.read_scalar().unwrap().to_u64().unwrap() as usize)
                                    .collect()
                            })
                            .collect(),
                    )
                }
                "dead_objects" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
            statearray: statearray.unwrap(),
        })
    }
}
