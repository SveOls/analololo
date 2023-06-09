use super::*;

#[derive(Debug)]
pub struct Party {}

impl Party {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, _) in inp.fields() {
            match key.read_str().as_ref() {
                "country" => {}
                "definition" => {}
                "votes" => {}
                "members_to_be" => {}
                "members" => {}
                "leader" => {}
                "activation_date" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {})
    }
    pub fn new_group(
        inp: ObjectReader<Utf8Encoding>,
    ) -> Result<HashMap<usize, Self>, Box<dyn Error>> {
        let mut ret = HashMap::new();

        for (key, _, value) in inp.fields() {
            ret.insert(key.read_str().parse()?, Party::new(value.read_object()?)?);
        }
        Ok(ret)
    }
}
