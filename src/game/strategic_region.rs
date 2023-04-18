use std::{collections::HashMap, error::Error, path::Path};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct StrategicRegion {
    name: String,
    num: usize,
    graphical_culture: Option<String>,
    states: Vec<String>,
    province_range: [usize; 2],
}

impl StrategicRegion {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn set_range(&mut self, inp: &mut Vec<super::StateRegion>, start: &mut usize) {
        self.province_range[0] = *start;
        for states in inp {
            if self.states.iter().any(|x| x == states.name()) {
                states.set_range(start);
            }
        }
        self.province_range[1] = *start;
    }
    pub fn read_range(&self) -> &[usize; 2] {
        &self.province_range
    }
    pub fn new(name: String, inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();
        ret.name = name;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "states" => {
                    ret.states = value
                        .read_array()?
                        .values()
                        .map(|x| x.read_string())
                        .try_collect()?
                }
                "graphical_culture" => ret.graphical_culture = Some(value.read_string()?),
                "capital_province" => {}
                "map_color" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }

        Ok(ret)
    }
    pub fn new_group(inp: &Path) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut ret = Vec::new();

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
                        let mut temp = Self::new(key.read_string(), value.read_object()?)?;
                        temp.num = i;
                        ret.push(temp);
                        i += 1;
                    }
                }
            }
        }

        Ok(ret)
    }
}
