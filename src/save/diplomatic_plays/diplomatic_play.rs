use super::*;

#[derive(Debug)]
pub struct DiplomaticPlay {}

impl DiplomaticPlay {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "type" => {}
                "initiators" => {}
                "involved" => {}
                "war_goals_attacker" => {}
                "war_goals_defender" => {}
                "country_records" => {}
                "targets" => {}
                "key" => {}
                "war" => {}
                "escalation" => {}
                "escalation_pause" => {}
                "initiator_maneuvers" => {}
                "target_maneuvers" => {}
                "casualties" => {}
                "state" => {}
                "strategic_region" => {}
                "target_leaning" => {}
                "initiator" => {}
                "target" => {}
                "start_date" => {}
                "end_date" => {}
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
                    .map(|x| DiplomaticPlay::new(x))
                    .transpose()?,
            );
        }
        Ok(ret)
    }
}
