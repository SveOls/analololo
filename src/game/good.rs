use std::{collections::HashMap, error::Error, path::{Path, PathBuf}};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct Good {
    num: usize,
    cost: i64,
    category: String,
    prestige_factor: i64,
    texture: PathBuf,
    tradeable: bool,
}

impl Good {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "texture" => ret.texture = PathBuf::from(value.read_scalar()?.to_string()),
                "cost" => ret.cost = value.read_scalar()?.to_i64()?,
                "category" => ret.category = value.read_string()?,
                "obsession_chance" => {},
                "prestige_factor" => ret.prestige_factor = value.read_scalar()?.to_i64()?,
                "consumption_tax_cost" => {},
                "convoy_cost_multiplier" => {},
                "tradeable" => ret.tradeable = value.read_scalar()?.to_bool()?,
                "fixed_price" => {},
                "traded_quantity" => {},
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }

        Ok(ret)
    }
    pub fn new_group(inp: &Path) -> Result<HashMap<String, Self>, Box<dyn Error>> {
        let mut ret = HashMap::new();

        let mut i = 0;
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
                        let mut temp = Self::new(value.read_object()?)?;
                        temp.num = i;
                        ret.insert(key.read_string(), temp);
                        i += 1;
                    }
                }
            }
        }

        Ok(ret)
    }
}
