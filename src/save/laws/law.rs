use super::*;

#[derive(Debug, Default, PartialEq)]
pub struct Law {
    law: String,
    country: usize,
    active: bool,
    replace: Option<String>,
    enactment_start_date: String,
    activation_date: String,
    enactment_last_checkpoint_or_stop_date: String,
    random_seed: i64,
    progress: f64,
}

impl Law {
    pub fn active(&self) -> bool {
        self.active
    }
    pub fn zeroinfo(&self) -> bool {
        let mut ret = Self::default();
        ret.law = self.law.clone();
        ret.country = self.country;
        self == &ret
    }
    pub fn country(&self) -> usize {
        self.country
    }
    pub fn law(&self) -> &String {
        &self.law
    }
    pub fn activation(&self) -> &String {
        &self.activation_date
    }
    pub fn replace(&self) -> &Option<String> {
        &self.replace
    }
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();
        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "law" => ret.law = value.read_string()?,
                "country" => ret.country = value.read_scalar()?.to_u64()? as usize,
                "active" => ret.active = value.read_scalar()?.to_bool()?,
                "replace" => ret.replace = Some(value.read_string()?),
                "activation_date" => ret.activation_date = value.read_string()?,
                "random_seed" => ret.random_seed = value.read_scalar()?.to_i64()?,
                "enactment_last_checkpoint_or_stop_date" => {
                    ret.enactment_last_checkpoint_or_stop_date = value.read_string()?
                }
                "enactment_start_date" => ret.enactment_start_date = value.read_string()?,
                "progress" => ret.progress = value.read_scalar()?.to_f64()?,
                "enactment_scope" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(ret)
    }
    pub fn new_group(
        inp: ObjectReader<Utf8Encoding>,
    ) -> Result<HashMap<usize, Option<Self>>, Box<dyn Error>> {
        let mut ret = HashMap::new();

        for (key, _, value) in inp.fields() {
            ret.insert(
                key.read_str().parse()?,
                value.read_object().ok().map(|x| Law::new(x)).transpose()?,
            );
        }
        Ok(ret)
    }
}
