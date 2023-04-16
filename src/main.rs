#![feature(iterator_try_collect)]

use std::{error::Error, io::Read};


use jomini::TextTape;

mod save;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    tester()?;
    println!("Hello, world :):)!");
    Ok(())
}


fn tester() -> Result<(), Box<dyn Error>> {
    // let data = std::fs::File::open("data/roman republic_1947_02_17.v3")?;
    let data = std::fs::File::open("data/papal states_1838_08_10.v3")?;
    let mut a = zip::ZipArchive::new(data)?;
    let mut info = Vec::new();
    a.by_name("gamestate")?.read_to_end(&mut info)?;
    let jomo = TextTape::from_slice(&info)?;

    let reader = jomo.utf8_reader();

    let _tits = save::Save::new(reader)?;

    Ok(())
}

