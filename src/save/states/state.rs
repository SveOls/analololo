
use jomini::text::ArrayReader;

use super::*;


#[derive(Debug)]
pub struct State {
    arable_land: i64,
    incorporation: Option<i64>,
    market: usize,
    infrastructure: i64,
    infrastructure_usage: Option<i64>,
    provinces: Vec<usize>,
    region: String,
    country: usize,
    capital: usize,
    colony_progress: Option<i64>,
    land: Option<i64>,
}

impl State {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {

        let mut arable_land = None;
        let mut incorporation = None;
        let mut market = None;
        let mut infrastructure = None;
        let mut infrastructure_usage = None;
        let mut provinces = Vec::new(); // todo
        let mut region = None;
        let mut country = None;
        let mut capital = None;
        let mut colony_progress = None;
        let mut land = None;

        for (key, _, value) in inp.fields() {
            // println!("{}", key.read_string());
            match key.read_str().as_ref() {
                "arable_land" => arable_land = Some(value.read_scalar()?.to_i64()?),
                "incorporation" => incorporation = Some((value.read_scalar()?.to_f64()? * 10e4) as i64),
                "migration_cache" => {},
                "slave_import_marker" => {},
                "market" => market = Some(value.read_scalar()?.to_u64()? as usize),
                "infrastructure" => infrastructure = Some((value.read_scalar()?.to_f64()? * 10e4) as i64),
                "infrastructure_usage" => infrastructure_usage = Some((value.read_scalar()?.to_f64()? * 10e4) as i64),
                "pop_needs" => {},
                "building_budget" => {},
                "variables" => {},
                "region" => region = Some(value.read_string()?),
                "provinces" => {},
                "pop_statistics" => {},
                "last_week_pop_migration_statistics" => {},
                "base_pop_bureaucracy_cost" => {},
                "capital" => capital = Some(value.read_scalar()?.to_u64()? as usize),
                "country" => country = Some(value.read_scalar()?.to_u64()? as usize),
                "colony_progress" => colony_progress = Some((value.read_scalar()?.to_f64()? * 10e4) as i64),
                "treaty_port" => {},
                "previous_owner" => {},
                "last_owner_change" => {},
                "mass_migration" => {},
                "slave_export_markers" => {},
                "last_week_slave_trade_statistics" => {},
                "timed_modifiers" => {},
                "pop_migration_statistics" => {},
                "land" => land = Some((value.read_scalar()?.to_f64()? * 10e4) as i64),
                "next_colony_province" => {},
                a => println!("\t\t\t\t\"{a}\" => {{}},")
            }
        }
        Ok(Self {
            land,
            arable_land: arable_land.unwrap(),
            incorporation,
            market: market.unwrap(),
            infrastructure: infrastructure.unwrap(),
            infrastructure_usage,
            provinces,
            region: region.unwrap(),
            country: country.unwrap(),
            capital: capital.unwrap(),
            colony_progress,
        })
    }
    pub fn new_group(inp: ObjectReader<Utf8Encoding>) -> Result<HashMap<usize, Option<Self>>, Box<dyn Error>> {

        let mut ret = HashMap::new();

        for (key, _, value) in inp.fields() {
            ret.insert(key.read_str().parse()?, value.read_object().ok().map(|x| State::new(x)).transpose()?);
        }
        Ok(ret)
    }
}
