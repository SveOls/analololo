use std::collections::HashMap;

use super::*;

mod journal_entry;
use journal_entry::JournalEntry;

#[allow(dead_code)]
pub struct JournalEntryManager {
    database: HashMap<usize, Option<JournalEntry>>,
}

impl JournalEntryManager {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut database = None;

        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "database" => database = Some(JournalEntry::new_group(value.read_object()?)?),
                "dead" => {}
                a => println!("\t\t\t\t\"{a}\" => !!{{}},"),
            }
        }
        Ok(Self {
            database: database.unwrap(),
        })
    }
}
