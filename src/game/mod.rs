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
mod needs;
use needs::Needs;
mod buy_package;
use buy_package::BuyPackage;
mod country;
use country::Country;
mod formable;
use formable::Formable;
mod law;
use law::Law;
mod law_group;
use law_group::LawGroup;
mod condition;

#[derive(Debug, Default)]
pub struct Game {
    production_methods: HashMap<String, ProductionMethod>,
    pops: HashMap<String, PopType>,
    religions: HashMap<String, Religion>,
    state_regions: Vec<StateRegion>,
    strategic_regions: Vec<StrategicRegion>,
    goods: Vec<Good>,
    needs: Vec<Needs>,
    countries: HashMap<String, Country>,
    formables: HashMap<String, Formable>,
    /// consumption of 300k working pops. Dependends count for 50%.
    /// note: SOL of 1 is in index, 0, SOL 2 in 1, and so on.
    buy_packages: Vec<BuyPackage>,
    laws: HashMap<String, Law>,
    law_groups: HashMap<String, LawGroup>,
}

impl Game {
    pub fn goods(&self) -> &Vec<Good> {
        &self.goods
    }
    pub fn buy_packages(&self) -> &Vec<BuyPackage> {
        &self.buy_packages
    }
    pub fn needs(&self) -> &Vec<Needs> {
        &self.needs
    }
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();

        ret.production_methods = ProductionMethod::new_group(
            &path.join("game").join("common").join("production_methods"),
        )?;
        ret.pops = PopType::new_group(&path.join("game").join("common").join("pop_types"))?;
        ret.religions = Religion::new_group(&path.join("game").join("common").join("religions"))?;
        ret.state_regions =
            StateRegion::new_group(&path.join("game").join("map_data").join("state_regions"))?;
        ret.strategic_regions = StrategicRegion::new_group(
            &path.join("game").join("common").join("strategic_regions"),
        )?;
        ret.goods = Good::new_group(&path.join("game").join("common").join("goods"))?;
        ret.needs = Needs::new_group(&path.join("game").join("common").join("pop_needs"))?;
        ret.buy_packages =
            BuyPackage::new_group(&path.join("game").join("common").join("buy_packages"))?;
        ret.countries =
            Country::new_group(&path.join("game").join("common").join("country_definitions"))?;
        ret.formables =
            Formable::new_group(&path.join("game").join("common").join("country_formation"))?;
        ret.laws = Law::new_group(&path.join("game").join("common").join("laws"))?;
        ret.law_groups = LawGroup::new_group(&path.join("game").join("common").join("law_groups"))?;

        // let mut a = ZipArchive::new(stuff)?;
        // let mut info = Vec::new();
        // a.by_name("gamestate")?.read_to_end(&mut info)?;
        let mut iter = 1;
        for i in &mut ret.strategic_regions {
            i.set_range(&mut ret.state_regions, &mut iter);
        }
        // println!("{:?}", ret.goods);
        // let inp = TextTape::from_slice(&info)?;
        // let inp = inp.utf8_reader();

        todo!();
        Ok(ret)
    }
}

//  "x12E54F" "x24CA48" "x4545BE" "xD8F441" "xD9A469" "xE041A0" "xE080A0"
// 20697 20699 20700 20701 20698 20695 20696
// 632 675
