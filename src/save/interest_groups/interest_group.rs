use super::*;

#[derive(Debug)]
pub struct InterestGroup {}

impl InterestGroup {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, _value) in inp.fields() {
            match key.read_str().as_ref() {
                "ideologies" => {}
                "name" => {}
                "country" => {}
                "definition" => {}
                "pops" => {}
                "leader" => {}
                "political_strength" => {}
                "radicals_political_strength" => {}
                "loyalists_political_strength" => {}
                "clout" => {}
                "clout_trend" => {}
                "in_government" => {}
                "approval" => {}
                "approval_state" => {}
                "influence_type" => {}
                "enabled_traits" => {}
                "variables" => {}
                "is_revolutionary" => {}
                "managed_status" => {}
                "timed_modifiers" => {}
                "party" => {}
                "influence_type_reason" => {}
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
                    .map(InterestGroup::new)
                    .transpose()?,
            );
        }
        Ok(ret)
    }
}
