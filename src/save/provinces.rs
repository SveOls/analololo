
use super::*;


pub struct Province {
    state: Option<usize>,
    building: Option<String>,
}

impl Province {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        let mut building = None;
        let mut state = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "state" => state = Some(value.read_scalar()?.to_u64()? as usize),
                "building" => building =  Some(value.read_string()?),
                a => println!("\t\t\t\t\"{a}\" => {{}},")
            }
        }
        Ok(Self { building, state })
    }
    pub fn new_group(inp: ObjectReader<Utf8Encoding>) -> Result<Vec<Self>, Box<dyn Error>> {

        let mut ret = Vec::new();

        for (key, _, value) in inp.fields() {
            if key.read_scalar().to_u64()? as usize != ret.len() {
                panic!();
            }
            ret.push(Province::new(value.read_object()?)?);
        }
        Ok(ret)
    }
}