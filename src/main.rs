#![feature(iterator_try_collect)]

use std::{error::Error, fs::File};

mod save;
mod scanner;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    tester()?;
    println!("Hello, world :):)!");
    Ok(())
}

fn tester() -> Result<(), Box<dyn Error>> {
    // let filename = "data/roman republic_1947_02_17.v3";
    let filename = "data/papal states_1838_08_10.v3";

    let tits = save::Save::new(File::open(filename)?)?;

    let holdo = scanner::Holder::new(tits);

    println!("{}", holdo.population(None, None));
    println!("{}", holdo.n_pops());
    println!("{}", holdo.global_gdp());
    let a = holdo.national_gdp();
    let mut b: Vec<(_, _)> = a.iter().collect();
    b.sort_by(|x, y| x.1.partial_cmp(y.1).unwrap());
    let relpop = holdo.population_religions();
    for i in relpop.iter() {
        println!("{} {}", i.0, i.1);
    }
    let culpop = holdo.population_cultures();
    for i in culpop.iter() {
        println!("{} {:?}", i.0, i.1);
    }
    let culpop = holdo.culture_religions();
    for i in culpop.iter() {
        println!("{} {:?}", i.0, i.1);
    }
    for i in culpop.iter() {
        let tot = i.1.1.values().sum::<i64>() as f64 / 100.0;
        println!("{}: {:.2?}", i.1.0, i.1.1.iter().map(|(k, &v)| (k, v as f64 / tot)).collect::<Vec<(_, _)>>());
    }
    let solcul = holdo.culture_sol();
    for i in solcul.iter() {
        println!("{:18}: {:.2}", i.1.0, i.1.1);
    }
    println!("{}", holdo.population(Some("jewish"), None));
    println!("{}", holdo.population(Some("jewish"), Some(12)));
    println!("{}", holdo.population(Some("jewish"), Some(58)));

    Ok(())
}
// 1046153240
// 34918
// 535394243.93779916

// 2753172682
// 665177
// 11437585696.624992