
use super::*;


pub struct Ironman {

}

impl Ironman {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "date" => {},
                "save_game" => {},
                "ironman" => {},
                "save_interval" => {},
                "storage" => {},
                a => println!("\t\t\t\t\"{a}\" => {{}},")
            }
        }
        Ok(Self { })
    }
}
