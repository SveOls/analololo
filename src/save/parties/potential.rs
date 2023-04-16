use super::*;

#[derive(Debug)]
pub struct Potential {}

impl Potential {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "country" => {}
                "potential" => {}
                "active" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {})
    }
    pub fn new_group(inp: ArrayReader<Utf8Encoding>) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut ret = Vec::new();

        for value in inp.values() {
            ret.push(Potential::new(value.read_object()?)?);
        }
        Ok(ret)
    }
}
