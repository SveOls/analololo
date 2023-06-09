use super::*;

#[derive(Debug)]
pub struct Hq {}

impl Hq {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, _value) in inp.fields() {
            match key.read_str().as_ref() {
                "garrison_counter" => {}
                "counters" => {}
                "region" => {}
                "troop_counter" => {}
                "country" => {}
                "type" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {})
    }
    pub fn new_group(
        inp: ObjectReader<Utf8Encoding>,
    ) -> Result<HashMap<usize, Option<Self>>, Box<dyn Error>> {
        let mut ret = HashMap::new();

        for (key, _, value) in inp.fields() {
            ret.insert(
                key.read_str().parse()?,
                value.read_object().ok().map(Hq::new).transpose()?,
            );
        }
        Ok(ret)
    }
}
