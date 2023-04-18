use super::*;

#[derive(Debug)]
pub struct Law {}

impl Law {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, _value) in inp.fields() {
            match key.read_str().as_ref() {
                "law" => {}
                "country" => {}
                "active" => {}
                "replace" => {}
                "activation_date" => {}
                "random_seed" => {}
                "enactment_last_checkpoint_or_stop_date" => {}
                "enactment_start_date" => {}
                "progress" => {}
                "enactment_scope" => {}
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
                value.read_object().ok().map(|x| Law::new(x)).transpose()?,
            );
        }
        Ok(ret)
    }
}
