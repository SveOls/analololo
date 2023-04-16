use super::*;

#[derive(Debug)]
pub struct CivilWarOne {}

impl CivilWarOne {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "origin_country" => {}
                "type" => {}
                "progress" => {}
                "uprisen_country" => {}
                "culture" => {}
                "insurrectionary_igs" => {}
                "political_movement" => {}
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
                value
                    .read_object()
                    .ok()
                    .map(|x| CivilWarOne::new(x))
                    .transpose()?,
            );
        }
        Ok(ret)
    }
}
