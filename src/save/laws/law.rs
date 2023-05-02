use super::*;

#[derive(Debug, Default)]
pub struct Law {
    law: String,
    country: usize,
    active: bool,
    replace: Option<String>,
    activation: String,
}

impl Law {
    pub fn active(&self) -> bool {
        self.active
    }
    pub fn country(&self) -> usize {
        self.country
    }
    pub fn law(&self) -> &String {
        &self.law
    }
    pub fn activation(&self) -> &String {
        &self.activation
    }
    pub fn replace(&self) -> &Option<String> {
        &self.replace
    }
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Option<Self>, Box<dyn Error>> {
        let mut ret = Self::default();
        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "law" => ret.law = value.read_string()?,
                "country" => ret.country = value.read_scalar()?.to_u64()? as usize,
                "active" => ret.active = value.read_scalar()?.to_bool()?,
                "replace" => ret.replace = Some(value.read_string()?),
                "activation_date" => ret.activation = value.read_string()?,
                "random_seed" => {}
                "enactment_last_checkpoint_or_stop_date" => {}
                "enactment_start_date" => {}
                "progress" => {}
                "enactment_scope" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        if ret.activation().is_empty() {
            Ok(None)
        } else {
            Ok(Some(ret))
        }
    }
    pub fn new_group(
        inp: ObjectReader<Utf8Encoding>,
    ) -> Result<HashMap<usize, Option<Self>>, Box<dyn Error>> {
        let mut ret = HashMap::new();

        for (key, _, value) in inp.fields() {
            ret.insert(
                key.read_str().parse()?,
                value.read_object().ok().map(|x| Law::new(x)).transpose()?.flatten(),
            );
        }
        Ok(ret)
    }
}
