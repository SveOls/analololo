use std::error::Error;

use jomini::{text::ObjectReader, Utf8Encoding};

#[derive(Debug)]
enum Condition {
    String(String),
    Float(f64),
    Integer(i64),
    Rec(Vec<Box<Self>>),
}

#[derive(Debug)]
pub enum Booli {
    Nand(Vec<Box<Self>>),
    Or(Vec<Box<Self>>),
    Not(Vec<Box<Self>>),
    Condition([String; 2]),
    ConditionBool((String, bool)),
}

impl Booli {
    // pub fn test(inp: ObjectReader<Utf8Encoding>) {
    //     for (key, _, val) in inp.fields() {
    //             if "change_allowed_trigger" == key.read_str().as_ref() {
    //                 println!("{:?}", Self::new(val.read_object().unwrap()))
    //             }
    //     }
    // }
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Vec<Box<Self>>, Box<dyn Error>> {
        let mut ret = Vec::new();
        for (key, _, val) in inp.fields() {
            match key.read_str().as_ref() {
                "NAND" => ret.push(Box::new(Self::Nand(Self::new(val.read_object()?)?))),
                "OR" => ret.push(Box::new(Self::Or(Self::new(val.read_object()?)?))),
                "NOT" => ret.push(Box::new(Self::Not(Self::new(val.read_object()?)?))),
                a => match (val.read_scalar().map(|z| z.to_bool()), val.read_string()) {
                    (Ok(Ok(b)), _) => ret.push(Box::new(Self::ConditionBool((a.to_string(), b)))),
                    (_, Ok(b)) => ret.push(Box::new(Self::Condition([a.to_owned(), b]))),
                    _ => panic!("{}", a),
                },
            }
        }
        Ok(ret)
    }
}
