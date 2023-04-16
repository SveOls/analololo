use super::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Pop {
    loyalist: i64,
    job: Option<String>,
    workforce: i64,
    dependents: i64,
    religion: String,
    literate: i64,
    growth_workforce: i64,
    growth_dependents: i64,
    wealth: i64,
    culture: usize,
    location: usize,
    weekly_budget: Vec<i64>,
}

impl Pop {
    pub fn size(&self) -> i64 {
        self.workforce() + self.dependents()
    }
    pub fn dependents(&self) -> i64 {
        self.dependents
    }
    pub fn workforce(&self) -> i64 {
        self.workforce
    }
    pub fn religion(&self) -> &str {
        &self.religion
    }
    pub fn culture(&self) -> usize {
        self.culture
    }
    pub fn wealth(&self) -> i64 {
        self.wealth
    }
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut loyalist = 0;
        let mut job = None;
        let mut wealth = None;
        let mut location = None;
        let mut workforce = 0;
        let mut growth_workforce = 0;
        let mut dependents = 0;
        let mut growth_dependents = 0;
        let mut literate = 0;
        let mut weekly_budget = None;
        let mut culture = None;
        let mut religion = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "loyalists_and_radicals" => loyalist = value.read_scalar()?.to_i64()?,
                "weekly_budget" => {
                    weekly_budget = Some(
                        value
                            .read_array()?
                            .values()
                            .map(|x| x.read_scalar().map(|x| (x.to_f64().unwrap() * 10e4) as i64))
                            .try_collect()?,
                    )
                }
                "partial_growth_wa" => {
                    growth_workforce = (value.read_scalar()?.to_f64()? * 10e4) as i64
                }
                "partial_growth_dn" => {
                    growth_dependents = (value.read_scalar()?.to_f64()? * 10e4) as i64
                }
                "is_accepted_culturally" => {}
                "is_discriminated" => {}
                "type" => job = Some(value.read_string()?),
                "size_wa" => workforce = value.read_scalar()?.to_i64()?,
                "size_dn" => dependents = value.read_scalar()?.to_i64()?,
                "location" => location = Some(value.read_scalar()?.to_u64()? as usize),
                "largest_interest_group" => {}
                "interest_group_support" => {}
                "culture" => culture = Some(value.read_scalar()?.to_u64()? as usize),
                "religion" => religion = Some(value.read_string()?),
                "literate" => literate = value.read_scalar()?.to_i64()?,
                "qualifications" => {}
                "wealth" => wealth = Some(value.read_scalar()?.to_i64()?),
                "wealth_progress" => {}
                "previous_quality_of_life" => {}
                "workplace" => {}
                "conversion_religion" => {}
                "is_accepted_religion" => {}
                "assimilation_culture" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {
            loyalist,
            job,
            workforce,
            dependents,
            religion: religion.unwrap(),
            culture: culture.unwrap(),
            weekly_budget: weekly_budget.unwrap(),
            literate,
            location: location.unwrap(),
            growth_workforce,
            growth_dependents,
            wealth: wealth.unwrap(),
        })
    }
    pub fn new_group(
        inp: ObjectReader<Utf8Encoding>,
    ) -> Result<HashMap<usize, Option<Self>>, Box<dyn Error>> {
        let mut ret = HashMap::new();

        for (key, _, value) in inp.fields() {
            ret.insert(
                key.read_str().parse()?,
                value.read_object().ok().map(|x| Pop::new(x)).transpose()?,
            );
        }
        Ok(ret)
    }
}
