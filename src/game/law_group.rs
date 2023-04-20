use std::{
    collections::HashMap,
    error::Error,
    path::Path,
};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct LawGroup {
    progressive_movement: f64,
    regressive_movement: f64,
    law_group_category: String,
    base_enactment_days: f64,
    enactment_approval_mult: f64,
}

impl LawGroup {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "progressive_movement_chance" => ret.progressive_movement = value.read_scalar()?.to_f64()?,
                "regressive_movement_chance" => ret.regressive_movement = value.read_scalar()?.to_f64()?,
                "law_group_category" => ret.law_group_category = value.read_string()?,
                "enactment_approval_mult" => ret.enactment_approval_mult = value.read_scalar()?.to_f64()?,
                "base_enactment_days" => ret.base_enactment_days = value.read_scalar()?.to_f64()?,
                "change_allowed_trigger" => {},
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
                    }
                }
            }
        }
        panic!();

        Ok(ret)
    }
}
