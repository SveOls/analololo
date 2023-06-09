use super::*;

#[allow(dead_code)]
#[derive(Debug)]
pub struct State {
    arable_land: i64,
    incorporation: Option<i64>,
    market: usize,
    infrastructure: i64,
    infrastructure_usage: i64,
    provinces: Vec<usize>,
    region: String,
    last_owner_change: Option<String>,
    country: usize,
    treaty_port: bool,
    capital: usize,
    colony_progress: Option<i64>,
    previous_owner: Option<String>,
    land: Option<i64>,
    next_colony_province: Option<usize>,
    // map of culture -> vector of needs
    // e.g. fish: 1, grain: 3, meat: 1.5 implies a consumption ratio of 1 fish, 3 grain, 1 meat for the "basic food" need, as meat costs 1.5x as much as grain and fish.
    pop_needs: HashMap<usize, Vec<Vec<(usize, f64)>>>,
}

impl State {
    pub fn market(&self) -> usize {
        self.market
    }
    pub fn country(&self) -> usize {
        self.country
    }
    pub fn region(&self) -> &String {
        &self.region
    }
    pub fn access(&self) -> f64 {
        if self.infrastructure > self.infrastructure_usage {
            1.0
        } else {
            self.infrastructure as f64 / self.infrastructure_usage as f64
        }
    }
    pub fn pop_needs(&self) -> &HashMap<usize, Vec<Vec<(usize, f64)>>> {
        &self.pop_needs
    }
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut arable_land = None;
        let mut incorporation = None;
        let mut market = None;
        let mut infrastructure = None;
        let mut infrastructure_usage = 0;
        let mut last_owner_change = None;
        let mut previous_owner = None;
        let mut treaty_port = false;
        let mut provinces = Vec::new();
        let mut region = None;
        let mut country = None;
        let mut next_colony_province = None;
        let mut capital = None;
        let mut colony_progress = None;
        let mut land = None;
        let mut pop_needs = HashMap::new();

        for (key, _, value) in inp.fields() {
            // println!("{}", key.read_string());
            match key.read_str().as_ref() {
                "arable_land" => arable_land = Some(value.read_scalar()?.to_i64()?),
                "incorporation" => {
                    incorporation = Some((value.read_scalar()?.to_f64()? * 10e4) as i64)
                }
                "migration_cache" => {}
                "slave_import_marker" => {}
                "market" => market = Some(value.read_scalar()?.to_u64()? as usize),
                "infrastructure" => {
                    infrastructure = Some((value.read_scalar()?.to_f64()? * 10e4) as i64)
                }
                "infrastructure_usage" => {
                    infrastructure_usage = (value.read_scalar()?.to_f64()? * 10e4) as i64
                }
                "pop_needs" => {
                    for (culture, _, inner_value) in value.read_object()?.fields() {
                        let mut temp = Vec::new();
                        for (a, _, b) in inner_value.read_object()?.fields() {
                            match a.read_str().as_ref() {
                                "pop_need_entry_data" => {
                                    for tt in b.read_array()?.values() {
                                        for (_, _, aa) in tt.read_object()?.fields() {
                                            temp.push(
                                                aa.read_object()?
                                                    .fields()
                                                    .map(|(m, _, n)| {
                                                        (
                                                            m.read_scalar().to_u64().unwrap()
                                                                as usize,
                                                            n.read_scalar()
                                                                .unwrap()
                                                                .to_f64()
                                                                .unwrap(),
                                                        )
                                                    })
                                                    .collect(),
                                            );
                                        }
                                    }
                                }
                                a => println!("\t\t\t\t\"{a}\" => {{}},"),
                            }
                        }
                        pop_needs.insert(culture.read_scalar().to_u64()? as usize, temp);
                    }
                }
                "building_budget" => {}
                "variables" => {}
                "region" => region = Some(value.read_string()?),
                "provinces" => {
                    provinces = value
                        .read_object()?
                        .fields()
                        .flat_map(|(_, _, x)| {
                            x.read_array()
                                .unwrap()
                                .values()
                                .map(|y| y.read_scalar().unwrap().to_u64().unwrap() as usize)
                        })
                        .collect()
                }
                "pop_statistics" => {}
                "last_week_pop_migration_statistics" => {}
                "base_pop_bureaucracy_cost" => {}
                "capital" => capital = Some(value.read_scalar()?.to_u64()? as usize),
                "country" => country = Some(value.read_scalar()?.to_u64()? as usize),
                "colony_progress" => {
                    colony_progress = Some((value.read_scalar()?.to_f64()? * 10e4) as i64)
                }
                "treaty_port" => treaty_port = value.read_scalar()?.to_bool()?,
                "previous_owner" => previous_owner = Some(value.read_scalar()?.to_string()),
                "last_owner_change" => last_owner_change = Some(value.read_scalar()?.to_string()),
                "mass_migration" => {}
                "slave_export_markers" => {}
                "last_week_slave_trade_statistics" => {}
                "timed_modifiers" => {}
                "pop_migration_statistics" => {}
                "land" => land = Some((value.read_scalar()?.to_f64()? * 10e4) as i64),
                "next_colony_province" => {
                    next_colony_province = Some(value.read_scalar()?.to_u64()? as usize)
                }
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {
            land,
            arable_land: arable_land.unwrap(),
            incorporation,
            next_colony_province,
            market: market.unwrap(),
            treaty_port,
            infrastructure: infrastructure.unwrap(),
            infrastructure_usage,
            provinces,
            region: region.unwrap(),
            country: country.unwrap(),
            capital: capital.unwrap(),
            colony_progress,
            previous_owner,
            pop_needs,
            last_owner_change,
        })
    }
    pub fn new_group(
        inp: ObjectReader<Utf8Encoding>,
    ) -> Result<HashMap<usize, Option<Self>>, Box<dyn Error>> {
        let mut ret = HashMap::new();

        for (key, _, value) in inp.fields() {
            ret.insert(
                key.read_str().parse()?,
                value
                    .read_object()
                    .ok()
                    .map(State::new)
                    .transpose()?,
            );
        }
        Ok(ret)
    }
}
