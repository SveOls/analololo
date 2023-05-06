use super::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Culture {
    name: String,
    seed: i64,
    homelands: Vec<String>,
}

impl Culture {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn homelands(&self) -> &[String] {
        &self.homelands
    }
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut name = None;
        let mut seed = None;
        let mut homelands = Vec::new();

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "type" => name = Some(value.read_string()?),
                "random_seed" => seed = Some(value.read_scalar()?.to_i64()?),
                "migration" => {}
                "obsessions" => {}
                "potential_migration" => {}
                "core_states" => {
                    homelands = value
                        .read_array()?
                        .values()
                        .map(|x| x.read_string())
                        .try_collect()?
                }
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {
            name: name.unwrap(),
            seed: seed.unwrap(),
            homelands,
        })
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
                    .map(Culture::new)
                    .transpose()?,
            );
        }
        Ok(ret)
    }
}
