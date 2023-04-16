
use super::*;


pub struct Variables {

}

impl Variables {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "data" => {},
                a => println!("\t\t\t\t\"{a}\" => {{}},")
            }
        }
        Ok(Self { })
    }
}
