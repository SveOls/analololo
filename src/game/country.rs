use std::{
    collections::HashMap,
    error::Error,
    path::{Path},
};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct Country {
    tier: String,
    country_type: String,
    cultures: Vec<String>,
    capital: String,
    religion: Option<String>,
    is_named_from_capital: Option<bool>,
    // color
}

impl Country {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "color" => {}
                "country_type" => ret.country_type = value.read_string()?,
                "tier" => ret.tier = value.read_string()?,
                "cultures" => {
                    ret.cultures = value
                        .read_array()?
                        .values()
                        .map(|x| x.read_string())
                        .try_collect()?
                }
                "is_named_from_capital" => {
                    ret.is_named_from_capital = Some(value.read_scalar()?.to_bool()?)
                }
                "religion" => ret.religion = Some(value.read_string()?),
                "capital" => ret.capital = value.read_string()?,
                "valid_as_home_country_for_separatists" => {}
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

        Ok(ret)
    }
}
