use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct Formable {
    use_culture_states: Option<bool>,
    is_major_formation: Option<bool>,
    required_states_faction: f64,
    states: Vec<String>,
    unification_play: Option<String>,
    leadership_play: Option<String>,
}

impl Formable {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "ai_will_do" => {}
                "use_culture_states" => {
                    ret.use_culture_states = Some(value.read_scalar()?.to_bool()?)
                }
                "required_states_fraction" => {
                    ret.required_states_faction = value.read_scalar()?.to_f64()?
                }
                "states" | "STATES" => {
                    ret.states = value
                        .read_array()?
                        .values()
                        .map(|x| x.read_string())
                        .try_collect()?
                }
                "possible" => {}
                "is_major_formation" => {
                    ret.is_major_formation = Some(value.read_scalar()?.to_bool()?)
                }
                "unification_play" => ret.unification_play = Some(value.read_string()?),
                "leadership_play" => ret.leadership_play = Some(value.read_string()?),
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
