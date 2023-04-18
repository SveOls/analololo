use std::{collections::HashMap, error::Error, path::Path};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct ProductionMethod {
    modifiers: HashMap<String, f64>,
}

impl ProductionMethod {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "building_modifiers" => {
                    for (k, _, stuff) in value
                        .read_object()?
                        .fields()
                        .flat_map(|x| x.2.read_object().unwrap().fields())
                    {
                        ret.modifiers
                            .insert(k.read_string(), stuff.read_scalar()?.to_f64()?);
                    }
                }
                "is_default" => {}
                "texture" => {}
                "unlocking_laws" => {}
                "disallowing_laws" => {}
                "country_modifiers" => {}
                "state_modifiers" => {}
                "low_pop_method" => {}
                "unlocking_religions" => {}
                "disallowing_religions" => {}
                "unlocking_technologies" => {}
                "timed_modifiers" => {}
                "unlocking_production_methods" => {}
                "unlocking_global_technologies" => {}
                "ai_weight" => {}
                "ai_value" => {}
                "pollution_generation" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        // println!("{:?}", ret);

        Ok(ret)
    }
    pub fn new_group(inp: &Path) -> Result<HashMap<String, Self>, Box<dyn Error>> {
        let mut ret = HashMap::new();

        for entry in std::fs::read_dir(inp)? {
            if let Ok(a) = entry {
                if a.path()
                    .extension()
                    .map(|x| x.to_str().unwrap() == "txt")
                    .unwrap_or(false)
                {
                    let temp = std::fs::read(a.path())?;
                    let c = TextTape::from_slice(&temp)?;
                    let d = c.utf8_reader();
                    for (key, _, value) in d.fields() {
                        ret.insert(key.read_string(), Self::new(value.read_object()?)?);
                    }
                }
            }
        }
        Ok(ret)
    }
}
