use std::collections::HashMap;

use super::*;

#[derive(Default, Debug)]
struct StringId {
    string: HashMap<usize, String>,
    id: HashMap<String, usize>,
}

impl StringId {
    fn get_string(&self, id: &usize) -> Option<&String> {
        self.string.get(id)
    }
    fn get_id(&self, string: &str) -> Option<&usize> {
        self.id.get(string)
    }
    fn insert(data: impl Iterator<Item = (usize, String)>) -> Self {
        let mut string = HashMap::new();
        let mut id = HashMap::new();
        for (i, s) in data {
            id.insert(s.clone(), i);
            string.insert(i, s);
        }
        Self { string, id }
    }
}
pub struct Holder {
    save: save::Save,
    game: game::Game,
    goods: StringId,
    buy_packages: StringId,
    needs: StringId,

}

impl Holder {
    pub fn new(save: save::Save, game: game::Game) -> Self {
        let goods = StringId::insert(
            game.goods()
                .iter()
                .enumerate()
                .map(|(i, x)| (i, x.name().to_owned())),
        );
        let buy_packages = StringId::insert(
            game.buy_packages()
                .iter()
                .enumerate()
                .map(|(i, x)| (i, x.name().to_owned())),
        );
        let needs = StringId::insert(
            game.needs()
                .iter()
                .enumerate()
                .map(|(i, x)| (i, x.name().to_owned())),
        );
        // panic!("{:?}", buy_packages);
        Self {
            save,
            game,
            goods,
            buy_packages,
            needs,
        }
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
    /// might not count slave needs properly.
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

        for pop in pops.values().filter_map(|x| x.as_ref()) {
            // for (a, pop) in pops.iter().filter_map(|x| x.1.as_ref().map(|y| (x.0, y))) {
            // println!("{}", a);
            // println!("{}", pop.location());
            // println!("{}", pop.culture());
            // println!("{:?}", states.get(&pop.location()).unwrap().as_ref().unwrap().pop_needs().keys().collect::<Vec<_>>());
            let scales = if let Some(a) = states
                .get(&pop.location())
                .map(|x| x.as_ref().map(|y| y.pop_needs().get(&pop.culture())))
                .flatten()
                .flatten()
            {
                a
            } else {
                continue;
            };
            let mut factor = (pop.workforce() as f64 + pop.dependents() as f64 / 2.0) / 10_000.0;
            if Some("peasants") == pop.job().map(|x| x.as_str()) {
                factor *= 0.1;
            }
            for (need_name, amount) in buy_packages[pop.wealth() as usize - 1].goods().iter() {
                let weights = scales.get(*self.needs.get_id(&need_name).unwrap()).unwrap();

                // panic!("{} {} {:?} {:?}", a, need_name, amount, weights);
                // let base_price = needs.iter().find(|x| x.name() == need_name).map(|y| goods.iter().find(|z| z.name() == y.default_good()).map(|o| o.price())).flatten().unwrap();
                let tot_weight = weights.iter().map(|x| x.1).fold(0.0, |acc, x| acc + x);
                for &(good, weight) in weights {
                    if tot_weight != 0.0 {
                        (*ret.entry(goods[good].name()).or_insert([0.0, 0.0]))[0] +=
                            (weight / tot_weight) * factor * amount / goods[good].price();
                    }
                }
            }
            // if *a == 11474 {panic!()}
        }
        // panic!();

        ret
    }
    /// does not include trade routes
    /// might not count slave needs properly.
    /// does not account for market access - assumes 100%.
    pub fn market_goods(&self) -> HashMap<usize, HashMap<&str, [f64; 2]>> {
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
            .filter_map(|x| x.1.as_ref().map(|x| (x.location(), x.goods_test())))
        {
            for j in i.1 {
                if i.0 == 0xFFFFFFFF {
                    break;
                }
                // initialize
                ret.entry(states.get(&i.0).unwrap().as_ref().unwrap().market())
                    .or_default();
                ret.entry(states.get(&i.0).unwrap().as_ref().unwrap().market())
                    .and_modify(|x: &mut HashMap<&str, [f64; 2]>| {
                        x.entry(goods[j.0].name()).or_insert([0.0, 0.0])[0] += j.1
                    });
                ret.entry(states.get(&i.0).unwrap().as_ref().unwrap().market())
                    .and_modify(|x: &mut HashMap<&str, [f64; 2]>| {
                        x.entry(goods[j.0].name()).or_insert([0.0, 0.0])[1] += j.2
                    });
            }
        }
        for pop in pops.values().filter_map(|x| x.as_ref()) {
            if pop.size() == 0 {
                continue;
            }
            let scales = if let Some(a) = states
                .get(&pop.location())
                .map(|x| x.as_ref().map(|y| y.pop_needs().get(&pop.culture())))
                .flatten()
                .flatten()
            {
                a
            } else {
                continue;
            };
            let mut factor = (pop.workforce() as f64 + pop.dependents() as f64 / 2.0) / 10_000.0;
            if Some("peasants") == pop.job().map(|x| x.as_str()) {
                factor *= 0.1;
            }
            for (need_name, amount) in buy_packages[pop.wealth() as usize - 1].goods().iter() {
                let weights = scales.get(*self.needs.get_id(&need_name).unwrap()).unwrap();
                let tot_weight = weights.iter().map(|x| x.1).fold(0.0, |acc, x| acc + x);
                // let tot_weight = 5.0;
                for (good, weight) in weights {
                    let entr = ret
                        .entry(
                            states
                                .get(&pop.location())
                                .unwrap()
                                .as_ref()
                                .unwrap()
                                .market(),
                        )
                        .or_default();

                    (*entr.entry(goods[*good].name()).or_insert([0.0, 0.0]))[0] +=
                        (weight / tot_weight) * factor * amount / goods[*good].price();
                }
            }
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
