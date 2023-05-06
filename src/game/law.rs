use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct Law {
    name: String,
    group: String,
    progressiveness: f64,
    possible_political_movements: Vec<String>,
    icon: PathBuf,
}

impl Law {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn icon(&self) -> &Path {
        self.icon.as_path()
    }
    pub fn group(&self) -> &String {
        &self.group
    }
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "group" => ret.group = value.read_string()?,
                "icon" => ret.icon = PathBuf::from(value.read_string()?),
                "progressiveness" => ret.progressiveness = value.read_scalar()?.to_f64()?,
                "modifier" => {}
                "on_enact" => {}
                "possible_political_movements" => {
                    ret.possible_political_movements = value
                        .read_array()?
                        .values()
                        .map(|x| x.read_string())
                        .try_collect()?
                }
                "pop_support" => {}
                "ai_will_do" => {}
                "unlocking_technologies" => {}
                "tariff_modifier_no_priority" => {}
                "tariff_modifier_export_priority" => {}
                "tariff_modifier_import_priority" => {}
                "disallowing_laws" => {}
                "institution" => {}
                "institution_modifier" => {}
                "on_activate" => {}
                "on_deactivate" => {}
                "tax_modifier_very_low" => {}
                "tax_modifier_low" => {}
                "tax_modifier_medium" => {}
                "tax_modifier_high" => {}
                "tax_modifier_very_high" => {}
                "revolution_state_weight" => {}
                "can_enact" => {}
                "cultural_acceptance_rule" => {}
                "religious_acceptance_rule" => {}
                "build_from_investment_pool" => {}
                "is_visible" => {}
                "unlocking_laws" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }

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
                        ret.entry(key.read_string())
                            .and_modify(|x| x.name = format!("{}", key.read_string()));
                    }
                }
            }
        }
        Ok(ret)
    }
}
