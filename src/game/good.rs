use std::{
    error::Error,
    path::{Path, PathBuf},
};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct Good {
    name: String,
    cost: f64,
    category: String,
    prestige_factor: i64,
    texture: PathBuf,
    tradeable: bool,
}

impl Good {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn price(&self) -> f64 {
        self.cost
    }
    pub fn new(name: String, inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();
        ret.name = name;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "texture" => ret.texture = PathBuf::from(value.read_scalar()?.to_string()),
                "cost" => ret.cost = value.read_scalar()?.to_f64()?,
                "category" => ret.category = value.read_string()?,
                "obsession_chance" => {}
                "prestige_factor" => ret.prestige_factor = value.read_scalar()?.to_i64()?,
                "consumption_tax_cost" => {}
                "convoy_cost_multiplier" => {}
                "tradeable" => ret.tradeable = value.read_scalar()?.to_bool()?,
                "fixed_price" => {}
                "traded_quantity" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }

        Ok(ret)
    }
    pub fn new_group(inp: &Path) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut ret = Vec::new();

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
                        ret.push(Self::new(key.read_string(), value.read_object()?)?);
                    }
                }
            }
        }

        Ok(ret)
    }
}
