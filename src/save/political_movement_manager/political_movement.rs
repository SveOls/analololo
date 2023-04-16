
use super::*;


#[derive(Debug)]
pub struct PoliticalMovement {
}

impl PoliticalMovement {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {


        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "law" => {},
                "start_date" => {},
                "timed_modifiers" => {},
                "interest_groups" => {},
                "country" => {},
                a => println!("\t\t\t\t\"{a}\" => {{}},")
            }
        }
        Ok(Self {
        })
    }
    pub fn new_group(inp: ObjectReader<Utf8Encoding>) -> Result<HashMap<usize, Option<Self>>, Box<dyn Error>> {

        let mut ret = HashMap::new();

        for (key, _, value) in inp.fields() {
            ret.insert(key.read_str().parse()?, value.read_object().ok().map(|x| PoliticalMovement::new(x)).transpose()?);
        }
        Ok(ret)
    }
}
