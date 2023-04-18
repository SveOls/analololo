use std::collections::HashMap;

use super::*;

pub struct Holder {
    save: save::Save,
    game: game::Game,
}

impl Holder {
    pub fn new(save: save::Save, game: game::Game) -> Self {
        Self { save, game }
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
    /// incorrect - c. 5% underestimation across the board
    pub fn global_gdp(&self) -> f64 {
        self.save
            .buildings()
            .database()
            .values()
            .filter_map(|x| x.as_ref())
            .fold(0.0, |acc, x| acc + x.goods_net())
            * 52.0
    }
    /// incorrect - c. 5% underestimation across the board
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
    pub fn global_goods(&self) -> HashMap<&str, [f64; 2]> {
        let mut ret = HashMap::new();
        let goods = self.game.goods();
        let needs = self.game.needs();
        let buy_packages = self.game.buy_packages();
        let pops = self.save.pops().database();
        let states = self.save.states().database();
        for i in self
            .save
            .buildings()
            .database()
            .iter()
            .filter_map(|x| x.1.as_ref().map(|x| x.goods_test()))
            .flatten()
        {
            (*ret.entry(goods[i.0].name()).or_insert([0.0, 0.0]))[0] += i.1;
            (*ret.entry(goods[i.0].name()).or_insert([0.0, 0.0]))[1] += i.2;
        }
        // for (a, pop) in pops.iter().filter_map(|x| x.1.as_ref().map(|y| (x.0, y))) {
        //     let scales = states.get(&pop.location()).map(|x| x.as_ref().map(|y| y.pop_needs().get(&pop.culture()))).flatten().flatten().unwrap();
        //     let factor = (pop.workforce() as f64 + pop.dependents() as f64 / 2.0) / 300_000.0;
        //     for (need_name, amount) in buy_packages[pop.wealth() as usize - 1].goods().iter() {
        //         let weights = needs.iter().position(|x| x.name() == need_name).map(|y| &scales[y]).unwrap();
        //         // panic!("{} {} {:?} {:?}", a, need_name, amount, weights);
        //         let base_price = needs.iter().find(|x| x.name() == need_name).map(|y| goods.iter().find(|z| z.name() == y.default_good()).map(|o| o.price())).flatten().unwrap();
        //         let tot_weight = weights.iter().map(|x| x.1).fold(0.0, |acc, x| acc + x);
        //         for (good, weight) in weights {
        //             // if *a == 6616 {
        //             //     println!("{}", good);
        //             //     println!("{}", weight);
        //             //     println!("{}", tot_weight);
        //             //     println!("{}", base_price);
        //             //     println!("{}", factor);
        //             //     println!("{}", amount);
        //             //     println!("{}\n", (weight / tot_weight) * (base_price / goods[*good].price()) * factor * amount);
        //             // }
        //             (*ret.entry(goods[*good].name()).or_insert([0.0, 0.0]))[0] += (weight / tot_weight) * (base_price / goods[*good].price()) * factor * amount;
        //         }
        //     }
        //     // if *a == 6616 {panic!()}
        // }
        for (a, pop) in pops.iter().filter_map(|x| x.1.as_ref().map(|y| (x.0, y))) {
            let scales = states.get(&pop.location()).map(|x| x.as_ref().map(|y| y.pop_needs().get(&pop.culture()))).flatten().flatten().unwrap();
            let factor = (pop.workforce() as f64 + pop.dependents() as f64 / 2.0) / 10_000.0;
            for (need_name, amount) in buy_packages[pop.wealth() as usize - 1].goods().iter() {
                let weights = needs.iter().position(|x| x.name() == need_name).map(|y| &scales[y]).unwrap();
                // panic!("{} {} {:?} {:?}", a, need_name, amount, weights);
                let base_price = needs.iter().find(|x| x.name() == need_name).map(|y| goods.iter().find(|z| z.name() == y.default_good()).map(|o| o.price())).flatten().unwrap();
                let tot_weight = weights.iter().map(|x| x.1).fold(0.0, |acc, x| acc + x);
                for (good, weight) in weights {
                    // if *a == 6616 {
                    //     println!("{}", good);
                    //     println!("{}", weight);
                    //     println!("{}", tot_weight);
                    //     println!("{}", base_price);
                    //     println!("{}", factor);
                    //     println!("{}", amount);
                    //     println!("{}\n", (weight / tot_weight) * (base_price / goods[*good].price()) * factor * amount);
                    // }
                    (*ret.entry(goods[*good].name()).or_insert([0.0, 0.0]))[0] += (weight / tot_weight) / goods[*good].price() * factor * amount;
                }
            }
            // if *a == 6616 {panic!()}
            
        }

        ret
    }
    pub fn market_gdp(&self) -> HashMap<usize, f64> {
        let mut buildings: HashMap<usize, f64> = HashMap::new();
        self.save
            .buildings()
            .database()
            .values()
            .filter_map(|x| x.as_ref())
            .for_each(|x| *buildings.entry(x.location()).or_default() += x.goods_net());
        let mut ret: HashMap<usize, f64> = HashMap::new();
        for (state, market) in self
            .save
            .states()
            .database()
            .iter()
            .filter_map(|(k, v)| v.as_ref().map(|x| (*k, x.market())))
        {
            *ret.entry(market).or_default() += buildings.get(&state).unwrap() * 52.0;
        }
        ret
    }
}
