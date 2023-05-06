use core::panic;

use super::*;

#[derive(Debug, Default)]
pub struct TradeRoute {
    owner: usize,
    // state ID
    trade_center: usize,
    // market ID of owner
    source: usize,
    // market ID of target
    target: usize,
    level: i64,
    direction: String,
    goods: String,
    traded: f64,
    revenue: f64,
    start_date: String,
}

impl TradeRoute {
    pub fn goods(&self) -> &String {
        &self.goods
    }
    pub fn source(&self) -> usize {
        self.source
    }
    pub fn target(&self) -> usize {
        self.target
    }
    pub fn traded(&self) -> f64 {
        if self.direction.as_str() == "import" {
            self.traded
        } else {
            -self.traded
        }
    }
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        let mut ret = Self::default();
        for (key, _, val) in inp.fields() {
            match key.read_str().as_ref() {
                "direction" => ret.direction = val.read_string()?,
                "goods" => ret.goods = val.read_string()?,
                "traded" => ret.traded = val.read_scalar()?.to_f64()?,
                "revenue" => ret.revenue = val.read_scalar()?.to_f64()?,
                "start_date" => ret.start_date = val.read_string()?,
                "owner" => ret.owner = val.read_scalar()?.to_u64()? as usize,
                "trade_center" => ret.trade_center = val.read_scalar()?.to_u64()? as usize,
                "source" => ret.source = val.read_scalar()?.to_u64()? as usize,
                "target" => ret.target = val.read_scalar()?.to_u64()? as usize,
                "level" => ret.level = val.read_scalar()?.to_i64()?,
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(ret)
    }
    pub fn new_group(
        inp: ObjectReader<Utf8Encoding>,
    ) -> Result<HashMap<usize, Option<Self>>, Box<dyn Error>> {
        let mut ret = HashMap::new();

        for (key, _, value) in inp.fields() {
            ret.insert(
                key.read_str().parse()?,
                value
                    .read_object()
                    .ok()
                    .map(|x| TradeRoute::new(x))
                    .transpose()?,
            );
        }
        Ok(ret)
    }
}
