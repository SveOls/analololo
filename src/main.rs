#![feature(iterator_try_collect)]

use std::{
    error::Error,
    fs::File,
    path::{Path, PathBuf},
};

mod game;
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
    // let filename = "data/prussia_1836_01_03.v3";
    let filename = "data/indian territory_1846_09_19.v3";
    let gameloc = PathBuf::from(r"/mnt/c/Program Files (x86)/Steam/steamapps/common/Victoria 3");
    // let gameloc = PathBuf::from(r"/mnt/c/Steam/steamapps/common/Victoria 3");

    let gam = game::Game::new(&gameloc)?;

    let tits = save::Save::new(File::open(filename)?)?;
        // panic!();

    let holdo = scanner::Holder::new(tits, gam);

    println!("{}", holdo.population(None, None));
    println!("{}", holdo.n_pops());
    println!("{}", holdo.global_gdp());
    let a = holdo.national_gdp();
    let mut b: Vec<(_, _)> = a.iter().collect();
    b.sort_by(|x, y| x.1.partial_cmp(y.1).unwrap());
    for i in b {
        println!("{:?}", i);
    }
    println!();
    for i in holdo.market_gdp() {
        println!("{:?}", i);
    }
    for i in holdo.global_goods() {
        println!("{}", i.0);
        println!("{:?}\n", i.1);
    }
    println!("???");
    for i in holdo.market_goods_access() {
        let mut sum = 0.0;
        println!("!! {}", i.0);
        for j in i.1 {
            sum += j.1[0];
            println!("{}", j.0);
            println!("{:?}\n", j.1);
        }
        // if sum > 2000.0 {
        //     break;
        // }
        if i.0 == 5 {
            break;
        }
    }
    // let gdp = holdo.national_gdp();
    // let gdp = gdp.get(&227);
    // println!("{:?}", gdp);
    // let relpop = holdo.population_religions();
    // for i in relpop.iter() {
    //     println!("{} {}", i.0, i.1);
    // }
    // let culpop = holdo.population_cultures();
    // for i in culpop.iter() {
    //     println!("{} {:?}", i.0, i.1);
    // }
    // let culpop = holdo.culture_religions();
    // for i in culpop.iter() {
    //     println!("{} {:?}", i.0, i.1);
    // }
    // for i in culpop.iter() {
    //     let tot = i.1 .1.values().sum::<i64>() as f64 / 100.0;
    //     println!(
    //         "{}: {:.2?}",
    //         i.1 .0,
    //         i.1 .1
    //             .iter()
    //             .map(|(k, &v)| (k, v as f64 / tot))
    //             .collect::<Vec<(_, _)>>()
    //     );
    // }
    // let solcul = holdo.culture_sol();
    // for i in solcul.iter() {
    //     println!("{:18}: {:.2}", i.1 .0, i.1 .1);
    // }
    // holdo.random_pop();
    // for i in holdo.state_goods() {
    //     println!("{}", i.0);
    //     println!("{:?}", i.1);
    //     if i.0 == 1 {
    //         break;
    //     }
    // }
    Ok(())
}
// 1046153240
// 34918
// 535394243.93779916

// 2753172682
// 665177
// 11437585696.624992
