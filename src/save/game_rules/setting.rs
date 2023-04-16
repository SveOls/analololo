use super::*;

#[derive(Debug)]
pub enum Setting {
    Other(String),
}

impl Setting {
    pub fn new(inp: Scalar) -> Result<Self, Box<dyn Error>> {
        Ok(Self::Other(inp.to_string()))
    }
    pub fn new_group(inp: ArrayReader<Utf8Encoding>) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut ret = Vec::new();

        for value in inp.values() {
            ret.push(Setting::new(value.read_scalar()?)?);
        }
        Ok(ret)
    }
}
