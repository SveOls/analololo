use super::*;

mod setting;
use setting::Setting;

#[allow(dead_code)]
pub struct GameRules {
    settings: Vec<Setting>,
}

impl GameRules {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut settings = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "setting" => settings = Some(Setting::new_group(value.read_array()?)?),
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            settings: settings.unwrap(),
        })
    }
}
