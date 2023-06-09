use super::*;

pub struct CountryRankings {}

impl CountryRankings {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, _) in inp.fields() {
            match key.read_str().as_ref() {
                "average_prestige" => {}
                "highest_prestige" => {}
                "country_rankings" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {})
    }
}
