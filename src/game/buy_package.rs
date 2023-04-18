use std::{
    error::Error,
    path::Path,
};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct BuyPackage {
    name: String,
    political_strength: f64,
    goods: Vec<(String, f64)>
}

impl BuyPackage {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn goods(&self) -> &Vec<(String, f64)> {
        &self.goods
    }
    pub fn new(name: String, inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();
        ret.name = name;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "political_strength" => ret.political_strength = value.read_scalar()?.to_f64()?,
                "goods" => {
                    ret.goods = value.read_object()?.fields().map(|x| (x.0.read_string(), x.2.read_scalar().unwrap().to_f64().unwrap())).collect()
                },
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
