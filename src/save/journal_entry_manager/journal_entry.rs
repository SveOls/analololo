use super::*;

#[derive(Debug)]
pub struct JournalEntry {}

impl JournalEntry {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, _value) in inp.fields() {
            match key.read_str().as_ref() {
                "active" => {}
                "bookmark" => {}
                "date_added" => {}
                "baseline_value" => {}
                "goal_value" => {}
                "objective_subgoal" => {}
                "target" => {}
                "variables" => {}
                "scope" => {}
                "type" => {}
                "country" => {}
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
                    .map(JournalEntry::new)
                    .transpose()?,
            );
        }
        Ok(ret)
    }
}
