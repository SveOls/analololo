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
    // 10_000 by default. Meaning: buy orders as defined by a pops SOL are scaled relative to 10_000.
    factor: f64,
}

impl Holder {
    pub fn testing(&self) {
        for i in self.state_goods().get(&1).unwrap() {
            println!("{} {:?}", i.0, i.1);
        }
        println!(
            "{}",
            self.save
                .states()
                .database()
                .get(&1)
                .unwrap()
                .as_ref()
                .unwrap()
                .region()
        );
    }
    pub fn random_pop(&self) {
        let mut pope = None;
        for (id, pop) in self
            .save
            .pops()
            .database()
            .iter()
            .filter_map(|(k, v)| v.as_ref().map(|x| (k, x)))
        {
            if *id == 14092 {
                pope = Some((id, pop));
                break;
            }
        }
        let (id, pop) = if let Some(a) = pope { a } else { return };
        println!("\nid: {id}");
        println!("religion: {}", pop.religion());
        println!(
            "culture: {} ({})",
            self.save
                .cultures()
                .database()
                .get(&pop.culture())
                .unwrap()
                .as_ref()
                .unwrap()
                .name(),
            pop.culture()
        );
        println!("job: {}", pop.job().unwrap());
        println!(
            "location: {} ({})",
            self.save
                .states()
                .database()
                .get(&pop.location())
                .unwrap()
                .as_ref()
                .unwrap()
                .region(),
            pop.location()
        );
        println!(
            "size: {}, {}, {}",
            pop.workforce(),
            pop.dependents(),
            pop.size()
        );
        println!(
            "literacy: {:.2}% ({})",
            100.0 * pop.literates() as f64 / pop.workforce() as f64,
            pop.literates()
        );
        println!("budget: {:?}", pop.budget());

        let scales = self
            .save
            .states()
            .database()
            .get(&pop.location())
            .unwrap()
            .as_ref()
            .unwrap()
            .pop_needs()
            .get(&pop.culture())
            .unwrap();
        let mut factor = (pop.workforce() as f64 + pop.dependents() as f64 / 2.0) / self.factor;
        let buy_packages = self.game.buy_packages();
        let goods = self.game.goods();
        if Some("peasants") == pop.job().map(|x| x.as_str()) {
            factor *= 0.1;
        }
        let mut hashma: HashMap<&str, f64> = HashMap::new();
        for (need_name, amount) in buy_packages[pop.wealth() as usize - 1].goods().iter() {
            let weights = scales.get(*self.needs.get_id(&need_name).unwrap()).unwrap();

            // panic!("{} {} {:?} {:?}", a, need_name, amount, weights);
            // let base_price = needs.iter().find(|x| x.name() == need_name).map(|y| goods.iter().find(|z| z.name() == y.default_good()).map(|o| o.price())).flatten().unwrap();
            let tot_weight = weights.iter().map(|x| x.1).fold(0.0, |acc, x| acc + x);
            for &(good, weight) in weights {
                if tot_weight != 0.0 {
                    *hashma.entry(goods[good].name()).or_default() +=
                        (weight / tot_weight) * factor * amount / goods[good].price();
                    // println!("{} {}", goods[good].name(), (weight / tot_weight) * factor * amount / goods[good].price());
                }
            }
        }
        for i in hashma {
            println!("{}: {:.4}", i.0, i.1)
        }
    }
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
            factor: 10_000.0,
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
        let mut ret: HashMap<&str, [f64; 2]> = HashMap::new();

        for (_, mut val) in self.state_goods().drain() {
            for (good, amount) in val.drain() {
                (*ret.entry(good).or_default())[0] += amount[0];
                (*ret.entry(good).or_default())[1] += amount[1];
            }
        }

        ret
    }
    /// includes market access and trade routes. Basically the numbers from the market overview screen.
    /// some pop needs (probably slaves) needs are still scuffed
    pub fn market_goods_full(&self) -> HashMap<usize, HashMap<&str, [f64; 2]>> {
        let mut ret = self.market_goods_access();
        for t in self
            .save
            .trade_routes()
            .database()
            .values()
            .filter_map(|x| x.as_ref())
        {
            ret.entry(t.source()).or_default();
            ret.entry(t.target()).or_default();
            if t.traded() > 0.0 {
                ret.entry(t.source())
                    .and_modify(|x: &mut HashMap<&str, [f64; 2]>| {
                        x.entry(&t.goods()).or_insert([0.0, 0.0])[1] += t.traded()
                    });
                ret.entry(t.target())
                    .and_modify(|x: &mut HashMap<&str, [f64; 2]>| {
                        x.entry(&t.goods()).or_insert([0.0, 0.0])[0] += t.traded()
                    });
            } else {
                ret.entry(t.source())
                    .and_modify(|x: &mut HashMap<&str, [f64; 2]>| {
                        x.entry(&t.goods()).or_insert([0.0, 0.0])[0] -= t.traded()
                    });
                ret.entry(t.target())
                    .and_modify(|x: &mut HashMap<&str, [f64; 2]>| {
                        x.entry(&t.goods()).or_insert([0.0, 0.0])[1] -= t.traded()
                    });
            }
        }
        ret
    }
    /// does not include trade routes
    /// might not count slave needs properly.
    /// does not account for market access - assumes 100%.
    pub fn market_goods(&self) -> HashMap<usize, HashMap<&str, [f64; 2]>> {
        let mut ret = HashMap::new();
        let states = self.save.states().database();
        for (k, v) in self.state_goods() {
            let a: &mut HashMap<&str, [f64; 2]> = ret
                .entry(
                    states
                        .get(&k)
                        .map(|x| x.as_ref())
                        .flatten()
                        .unwrap()
                        .market(),
                )
                .or_default();
            for (ik, iv) in v {
                (*a.entry(ik).or_default())[0] += iv[0];
                (*a.entry(ik).or_default())[1] += iv[1];
            }
        }
        ret
    }
    /// does not include trade routes
    /// might not count slave needs properly.
    pub fn market_goods_access(&self) -> HashMap<usize, HashMap<&str, [f64; 2]>> {
        let mut ret = HashMap::new();
        let states = self.save.states().database();
        for (k, v) in self.state_goods() {
            let a: &mut HashMap<&str, [f64; 2]> = ret
                .entry(
                    states
                        .get(&k)
                        .map(|x| x.as_ref())
                        .flatten()
                        .unwrap()
                        .market(),
                )
                .or_default();
            for (ik, iv) in v {
                (*a.entry(ik).or_default())[0] += iv[0]
                    * states
                        .get(&k)
                        .map(|x| x.as_ref())
                        .flatten()
                        .unwrap()
                        .access();
                (*a.entry(ik).or_default())[1] += iv[1]
                    * states
                        .get(&k)
                        .map(|x| x.as_ref())
                        .flatten()
                        .unwrap()
                        .access();
            }
        }
        ret
    }
    pub fn state_goods(&self) -> HashMap<usize, HashMap<&str, [f64; 2]>> {
        let mut ret = HashMap::new();
        let goods = self.game.goods();
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
                ret.entry(i.0).or_default();
                ret.entry(i.0)
                    .and_modify(|x: &mut HashMap<&str, [f64; 2]>| {
                        x.entry(goods[j.0].name()).or_insert([0.0, 0.0])[0] += j.1
                    });
                ret.entry(i.0)
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
            let mut factor = (pop.workforce() as f64 + pop.dependents() as f64 / 2.0) / self.factor;
            if Some("peasants") == pop.job().map(|x| x.as_str()) {
                factor *= 0.1;
            }
            for (need_name, amount) in buy_packages[pop.wealth() as usize - 1].goods().iter() {
                let weights = scales.get(*self.needs.get_id(&need_name).unwrap()).unwrap();
                let tot_weight = weights.iter().map(|x| x.1).fold(0.0, |acc, x| acc + x);
                // let tot_weight = 5.0;
                for (good, weight) in weights {
                    let entr = ret.entry(pop.location()).or_default();

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
    pub fn country_law_history(
        &self,
        filter_never_used: bool,
    ) -> HashMap<usize, Vec<&save::laws::Law>> {
        self.save
            .laws()
            .database()
            .values()
            .filter_map(|x| x.as_ref())
            .filter(|x| !(x.zeroinfo() && filter_never_used))
            .map(|law| (law.country(), law))
            .fold(HashMap::new(), |mut acc, x| {
                acc.entry(x.0).or_insert_with(Vec::new).push(x.1);
                acc
            })
    }
    /// for key = category, returns all groups and laws in the category.
    /// for key = group, returns all laws in the group, as well as the category it belongs to.
    /// for key = law, returns the category and group it belongs to.
    pub fn law_context(&self) -> HashMap<&str, (&str, Vec<&game::LawGroup>, Vec<&game::Law>)> {
        self.game
            .laws()
            .iter()
            .map(|x| (x.0, x.1, self.game.law_groups().get(x.1.group()).unwrap()))
            .fold(HashMap::new(), |mut acc, (k, v2, v1)| {
                acc.entry(k.as_str())
                    .or_insert_with(|| (v1.category().as_str(), vec![v1], vec![v2]));
                acc.entry(v2.group())
                    .or_insert_with(|| (v1.category().as_str(), vec![v1], Vec::new()))
                    .2
                    .push(v2);
                acc.entry(v1.category())
                    .or_insert_with(|| (v1.category().as_str(), Vec::new(), Vec::new()))
                    .1
                    .push(v1);
                acc.entry(v1.category())
                    .or_insert_with(|| (v1.category().as_str(), Vec::new(), Vec::new()))
                    .2
                    .push(v2);

                acc
            })
    }
    pub fn localization(&self) -> &HashMap<String, game::Localization> {
        self.game.localization()
    }
}
