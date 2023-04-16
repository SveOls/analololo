use super::*;

#[derive(Debug)]
pub struct War {}

impl War {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, _) in inp.fields() {
            match key.read_str().as_ref() {
                "name" => {}
                "attacker_peace_deal" => {}
                "defender_peace_deal" => {}
                "dead" => {}
                "days_since_exhaustion" => {}
                "diplomatic_play" => {}
                "start_date" => {}
                "peace_date" => {}
                "war_participants" => {}
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
                value.read_object().ok().map(|x| War::new(x)).transpose()?,
            );
        }
        Ok(ret)
    }
}
