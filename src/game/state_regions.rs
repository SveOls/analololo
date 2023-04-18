use std::{collections::HashMap, error::Error, path::Path};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct StateRegion {
    name: String,
    id: usize,
    num: usize,
    subsistence_building: Option<String>,
    arable_land: Option<i64>,
    arable_resources: Vec<String>,
    naval_exit_id: Option<usize>,
    traits: Vec<String>,
    provinces: Vec<String>,
    // capped_resources: HashMap<String, i64>,
    province_range: [usize; 2],
}

impl StateRegion {
    pub fn set_range(&mut self, inp: &mut usize) {
        self.province_range[0] = *inp;
        *inp += self.provinces.len();
        self.province_range[1] = *inp;
    }
    pub fn read_range(&self) -> &[usize; 2] {
        &self.province_range
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn new(name: String, inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();
        ret.name = name;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "id" => ret.id = value.read_scalar()?.to_u64()? as usize,
                "naval_exit_id" => {
                    ret.naval_exit_id = Some(value.read_scalar()?.to_u64()? as usize)
                }
                "arable_land" => ret.arable_land = Some(value.read_scalar()?.to_i64()?),
                "subsistence_building" => ret.subsistence_building = Some(value.read_string()?),
                "arable_resources" => {
                    ret.arable_resources = value
                        .read_array()?
                        .values()
                        .map(|x| x.read_string())
                        .try_collect()?
                }
                "traits" => {
                    ret.traits = value
                        .read_array()?
                        .values()
                        .map(|x| x.read_string())
                        .try_collect()?
                }
                "capped_resources" => {}
                "provinces" => {
                    ret.provinces = value
                        .read_array()?
                        .values()
                        .map(|x| x.read_string())
                        .try_collect()?
                }
                "impassable" => {}
                "prime_land" => {}
                "resource" => {}
                "city" => {}
                "port" => {}
                "farm" => {}
                "mine" => {}
                "wood" => {}
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
