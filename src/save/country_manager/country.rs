
use super::*;

#[derive(Debug, Default)]
pub struct Country {
    states: Vec<usize>,
}

impl Country {
    pub fn states(&self) -> &[usize] {
        &self.states
    }
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        
        let mut ret = Self::default();

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "cultures" => {}
                "religion" => {}
                "pop_statistics" => {}
                "pop_radicals_and_loyalists_statistics" => {}
                "states" => ret.states = value.read_array()?.values().map(|x| x.read_scalar().unwrap().to_u64().unwrap() as usize).collect(),
                "timed_modifiers" => {}
                "outliner_data" => {}
                "government_queue" => {}
                "default_on_action_counter" => {}
                "convoys_produced" => {}
                "previous_definitions" => {}
                "construction_paused" => {}
                "mil_salaries" => {}
                "strategic_objectives" => {}
                "new_production_methods" => {}
                "in_default" => {}
                "convoys_from_subjects" => {}
                "diplomatic_pacts_counter" => {}
                "counters" => {}
                "is_main_tag" => {}
                "definition" => {}
                "government" => {}
                "ruler" => {}
                "civil_war" => {}
                "heir" => {}
                "capital" => {}
                "budget" => {}
                "gdp" => {}
                "prestige" => {}
                "variables" => {}
                "taxed_goods" => {}
                "literacy" => {}
                "avgsoltrend" => {}
                "tax_level" => {}
                "tariffs" => {}
                "country_type" => {}
                "private_queue" => {}
                "claims" => {}
                "market_capital" => {}
                "map_color" => {}
                "timed_enactment_modifiers" => {}
                "game_over_date" => {}
                "dead" => {}
                "infamy" => {}
                "trends_initialized" => {}
                "salaries" => {}
                "declared_interests" => {}
                "dynamic_country_name" => {}
                "dynamic_country_adjective" => {}
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
                value
                    .read_object()
                    .ok()
                    .map(|x| Country::new(x))
                    .transpose()?,
            );
        }
        Ok(ret)
    }
}
