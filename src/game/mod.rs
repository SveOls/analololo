use std::{collections::HashMap, error::Error, path::Path};

mod production_method;
use production_method::ProductionMethod;
mod pops;
use pops::PopType;
mod religion;
use religion::Religion;
mod state_regions;
use state_regions::StateRegion;
mod strategic_region;
use strategic_region::StrategicRegion;
mod good;
use good::Good;

#[derive(Debug, Default)]
pub struct Game {
    production_methods: HashMap<String, ProductionMethod>,
    pops: HashMap<String, PopType>,
    religions: HashMap<String, Religion>,
    state_regions: HashMap<String, StateRegion>,
    strategic_regions: HashMap<String, StrategicRegion>,
    goods: HashMap<String, Good>,
}

impl Game {
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();

        ret.production_methods = ProductionMethod::new_group(
            &path.join("game").join("common").join("production_methods"),
        )?;
        ret.pops = PopType::new_group(&path.join("game").join("common").join("pop_types"))?;
        ret.religions = Religion::new_group(&path.join("game").join("common").join("religions"))?;
        ret.state_regions =
            StateRegion::new_group(&path.join("game").join("map_data").join("state_regions"))?;
        ret.strategic_regions =
            StrategicRegion::new_group(&path.join("game").join("common").join("strategic_regions"))?;
        ret.goods = Good::new_group(&path.join("game").join("common").join("goods"))?;

        // let mut a = ZipArchive::new(stuff)?;
        // let mut info = Vec::new();
        // a.by_name("gamestate")?.read_to_end(&mut info)?;
        let mut iter = 1;
        for (_, i) in &mut ret.strategic_regions {
            iter = i.update(&ret.state_regions, iter);
        }
        println!("{:?}", ret.goods);
        // let inp = TextTape::from_slice(&info)?;
        // let inp = inp.utf8_reader();



        todo!()
    }
}
