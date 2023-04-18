use std::collections::HashMap;

use super::*;

pub struct Holder {
    save: save::Save,
}

impl Holder {
    pub fn new(save: save::Save) -> Self {
        Self { save }
    }
    pub fn population(&self, religion: Option<&str>, culture: Option<usize>) -> i64 {
        self.save
            .pops()
            .database()
            .values()
            .filter_map(|x| x.as_ref())
            .filter(|x| religion.map(|rel| rel == x.religion()).unwrap_or(true))
            .filter(|x| culture.map(|cul| cul == x.culture()).unwrap_or(true))
            .fold(0, |acc, pop| acc + pop.size())
    }
    pub fn n_pops(&self) -> usize {
        self.save
            .pops()
            .database()
            .values()
            .filter_map(|x| x.as_ref())
            .count()
    }
    pub fn population_religions(&self) -> HashMap<&str, i64> {
        let mut ret = HashMap::new();
        self.save
            .pops()
            .database()
            .values()
            .filter_map(|x| x.as_ref())
            .for_each(|x| *ret.entry(x.religion()).or_default() += x.size());
        ret
    }
    pub fn population_cultures(&self) -> HashMap<usize, (String, i64)> {
        let mut ret: HashMap<usize, (String, i64)> = self
            .save
            .cultures()
            .database()
            .iter()
            .filter_map(|(&k, v)| v.as_ref().map(|x| (k, (x.name().to_owned(), 0))))
            .collect();
        self.save
            .pops()
            .database()
            .values()
            .filter_map(|x| x.as_ref())
            .for_each(|x| {
                ret.entry(x.culture())
                    .and_modify(|(_, pop)| *pop += x.size());
            });
        ret
    }
    pub fn culture_religions(&self) -> HashMap<usize, (String, HashMap<&str, i64>)> {
        let mut ret: HashMap<usize, (String, HashMap<&str, i64>)> = self
            .save
            .cultures()
            .database()
            .iter()
            .filter_map(|(&k, v)| {
                v.as_ref()
                    .map(|x| (k, (x.name().to_owned(), HashMap::new())))
            })
            .collect();
        self.save
            .pops()
            .database()
            .values()
            .filter_map(|x| x.as_ref())
            .for_each(|x| {
                ret.entry(x.culture())
                    .and_modify(|(_, pop)| *pop.entry(x.religion()).or_default() += x.size());
            });
        ret
    }
    pub fn culture_sol(&self) -> HashMap<usize, (String, f64)> {
        let mut solo: HashMap<usize, (String, i64)> = self
            .save
            .cultures()
            .database()
            .iter()
            .filter_map(|(&k, v)| v.as_ref().map(|x| (k, (x.name().to_owned(), 0))))
            .collect();
        self.save
            .pops()
            .database()
            .values()
            .filter_map(|x| x.as_ref())
            .for_each(|x| {
                solo.entry(x.culture())
                    .and_modify(|(_, pop)| *pop += x.size() * x.wealth());
            });

        self.population_cultures()
            .into_iter()
            .map(|(k, v)| (k, (v.0, solo.get(&k).unwrap().1 as f64 / v.1 as f64)))
            .collect()
    }
    // incorrect
    pub fn global_gdp(&self) -> f64 {
        self.save
            .buildings()
            .database()
            .values()
            .filter_map(|x| x.as_ref())
            .fold(0.0, |acc, x| acc + x.goods_net())
            * 52.0
    }
    // incorrect
    pub fn national_gdp(&self) -> HashMap<usize, f64> {
        let mut buildings: HashMap<usize, f64> = HashMap::new();
        self.save
            .buildings()
            .database()
            .values()
            .filter_map(|x| x.as_ref())
            .for_each(|x| *buildings.entry(x.location()).or_default() += x.goods_net());
        let mut ret: HashMap<usize, f64> = HashMap::new();
        for (country, states) in self
            .save
            .countries()
            .database()
            .iter()
            .filter_map(|(k, v)| v.as_ref().map(|x| (*k, x.states())))
        {
            let mut count = 0.0;
            for i in states {
                count += *buildings.get(i).unwrap_or(&0.0);
            }
            ret.insert(country, count * 52.0);
        }
        ret
    }
}
