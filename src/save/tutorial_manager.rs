use super::*;

pub struct TutorialManager {}

impl TutorialManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "active" => {}
                "completed_lessons" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {})
    }
}
