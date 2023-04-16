
use super::*;

mod ai;
use ai::InnerAi;

pub struct Ai {}

impl Ai {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut stuff = Vec::new();

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "ai" => stuff.push(InnerAi::new(value.read_object()?)?),
                "last_ai_spending_update_index" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {})
    }
}
