use super::*;

pub struct Counters {}

impl Counters {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "command" => {}
                "canal_connections" => {}
                "tick" => {}
                "week" => {}
                "province_owner" => {}
                "province_theater" => {}
                "state_provinces" => {}
                "wars" => {}
                "diplomatic_play_participants" => {}
                "markets" => {}
                "create_destroy_markets" => {}
                "fully_executed_commands" => {}
                "modifiers" => {}
                "constructions_progressed" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {})
    }
}
