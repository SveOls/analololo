use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct Religion {
    texture: PathBuf,
    traits: Vec<String>,
    taboos: Vec<String>,
}

impl Religion {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "texture" => ret.texture = PathBuf::from(value.read_scalar()?.to_string()),
                "traits" => {
                    ret.traits = value
                        .read_array()?
                        .values()
                        .map(|x| x.read_scalar().map(|x| x.to_string()))
                        .try_collect()?
                }
                "taboos" => {
                    ret.taboos = value
                        .read_array()?
                        .values()
                        .map(|x| x.read_scalar().map(|x| x.to_string()))
                        .try_collect()?
                }
                "color" => {}
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
