use super::*;

pub struct Metadata {}

impl Metadata {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "save_game_version" => {}
                "version" => {}
                "achievement_eligibility" => {}
                "game_date" => {}
                "real_date" => {}
                "name" => {}
                "rank" => {}
                "flag" => {}
                "dlcs" => {}
                "game_rules" => {}
                "ironman" => {}
                "number_of_players" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {})
    }
}
