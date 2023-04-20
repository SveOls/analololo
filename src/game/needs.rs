use std::{error::Error, path::Path};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct Needs {
    name: String,
    default: String,
    entries: Vec<(String, [f64; 3])>,
}

impl Needs {
    pub fn entries(&self) -> &Vec<(String, [f64; 3])> {
        &self.entries
    }
    pub fn default_good(&self) -> &str {
        &self.default
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn new(name: String, inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();
        ret.name = name;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "default" => ret.default = value.read_string()?,
                "entry" => {
                    let mut entry = (String::new(), [0.0; 3]);
                    for (inner_key, _, inner_value) in value.read_object()?.fields() {
                        match inner_key.read_str().as_ref() {
                            "goods" => entry.0 = inner_value.read_string()?,
                            "weight" => entry.1[0] = inner_value.read_scalar()?.to_f64()?,
                            "max_weight" => entry.1[1] = inner_value.read_scalar()?.to_f64()?,
                            "min_weight" => entry.1[2] = inner_value.read_scalar()?.to_f64()?,
                            a => println!("\t\t\t\t\t\t\t\"{a}\" => {{}},"),
                        }
                    }
                    ret.entries.push(entry);
                }
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
