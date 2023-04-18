use std::{collections::HashMap, error::Error, path::Path};

use jomini::{text::ObjectReader, TextTape, Utf8Encoding};

#[derive(Debug, Default)]
pub struct PopType {
    unemployment: bool,
    is_slave: bool,
    can_always_hire: bool,
    strata: String,
}

impl PopType {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "unemployment" => ret.unemployment = value.read_scalar()?.to_bool()?,
                "is_slave" => ret.is_slave = value.read_scalar()?.to_bool()?,
                "can_always_hire" => ret.can_always_hire = value.read_scalar()?.to_bool()?,
                "strata" => ret.strata = value.read_scalar()?.to_string(),
                "start_quality_of_life" => {}
                "working_adult_ratio" => {}
                "wage_weight" => {}
                "dependent_wage" => {}
                "political_engagement_base" => {}
                "political_engagement_literacy_factor" => {}
                "political_engagement_mult" => {}
                "portrait_age" => {}
                "portrait_pose" => {}
                "portrait_is_female" => {}
                "literacy_target" => {}
                "unemployment_wealth" => {}
                "qualifications_growth_desc" => {}
                "education_access" => {}
                "qualifications" => {}
                "subsistence_income" => {}
                "ignores_employment_proportionality" => {}
                "consumption_mult" => {}
                "texture" => {}
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
