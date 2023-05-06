use super::*;

#[derive(Debug)]
pub struct Battle {}

impl Battle {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, _) in inp.fields() {
            match key.read_str().as_ref() {
                "attacker_province" => {}
                "is_naval_invasion" => {}
                "status" => {}
                "name" => {}
                "start_date" => {}
                "attacker_start_ce" => {}
                "defender_start_ce" => {}
                "attacker_end_ce" => {}
                "defender_end_ce" => {}
                "attacker_start_battalions" => {}
                "defender_start_battalions" => {}
                "attacker_ending_battalions" => {}
                "defender_ending_battalions" => {}
                "attacker_start_manpower" => {}
                "defender_start_manpower" => {}
                "num_captured_provinces" => {}
                "capturing_country" => {}
                "lost_provinces_country" => {}
                "attacker_trend" => {}
                "defender_trend" => {}
                "war" => {}
                "front" => {}
                "province" => {}
                "end_date" => {}
                "attacker_still_fighting_trend" => {}
                "defender_still_fighting_trend" => {}
                "attacker_manpower_trend" => {}
                "defender_manpower_trend" => {}
                "battle_data" => {}
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
                    .map(Battle::new)
                    .transpose()?,
            );
        }
        Ok(ret)
    }
}
