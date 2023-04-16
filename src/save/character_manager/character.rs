use super::*;

#[derive(Debug)]
pub struct Character {}

impl Character {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, _value) in inp.fields() {
            match key.read_str().as_ref() {
                "is_traveling" => {}
                "traits" => {}
                "expected_trait_value" => {}
                "religion" => {}
                "first_name" => {}
                "last_name" => {}
                "culture" => {}
                "role" => {}
                "order" => {}
                "birth_date" => {}
                "cooldown_repair_date" => {}
                "last_health_value" => {}
                "rank" => {}
                "ideology" => {}
                "travel_start_date" => {}
                "travel_end_date" => {}
                "noble" => {}
                "timed_modifiers" => {}
                "variables" => {}
                "is_female" => {}
                "dna" => {}
                "previous_order" => {}
                "is_mobilized" => {}
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
                    .map(|x| Character::new(x))
                    .transpose()?,
            );
        }
        Ok(ret)
    }
}
