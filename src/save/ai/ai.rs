use super::*;

#[derive(Debug)]
pub struct InnerAi {}

impl InnerAi {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, _value) in inp.fields() {
            match key.read_str().as_ref() {
                "ai_goal" => {}
                "country_goal" => {}
                "ai_strategy" => {}
                "potential_money_spending" => {}
                "potential_authority_spending_changes" => {}
                "potential_building_constructions" => {}
                "potential_private_investment_building_constructions" => {}
                "building_subsidies_changed" => {}
                "building_priorities_changed" => {}
                "private_investment_priorities_changed" => {}
                "spending_variables" => {}
                "random_seed" => {}
                "country" => {}
                "enabled" => {}
                "ai_memories" => {}
                "cached_ai_weights" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {})
    }
}
