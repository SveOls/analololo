use std::{
    collections::HashMap,
    error::Error,
    path::{Path},
};

use walkdir;


#[derive(Debug, Default)]
pub struct Localization {
    data: String,
}

impl Localization {
    pub fn obtain(&self) -> &String {
        &self.data
    }
    pub fn new(inp: &str) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();
        ret.data = inp.trim_matches('"').to_owned();

        Ok(ret)
    }
    pub fn new_group(inp: &Path) -> Result<HashMap<String, Self>, Box<dyn Error>> {
        let mut ret = HashMap::new();
        for entry in walkdir::WalkDir::new(inp).into_iter() {
            if let Ok(a) = entry {
                if a.path()
                    .extension()
                    .map(|x| x.to_str().unwrap() == "yml")
                    .unwrap_or(false)
                {
                    let temp = std::fs::read_to_string(a.path())?;
                    for (key, val) in temp.lines().filter_map(|x| x.trim().split_once(' ')).filter(|x| !x.0.starts_with('#')) {
                        ret.insert(key.split(':').next().unwrap().to_owned(), Self::new(val)?);
                    }
                }
            }
        }
        Ok(ret)
    }
}
