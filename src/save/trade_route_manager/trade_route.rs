use super::*;

#[derive(Debug)]
pub struct TradeRoute {}

impl TradeRoute {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "direction" => {}
                "goods" => {}
                "traded" => {}
                "revenue" => {}
                "start_date" => {}
                "owner" => {}
                "trade_center" => {}
                "source" => {}
                "target" => {}
                "level" => {}
                a => println!("\t\t\t\t\"{a}\" => {{}},"),
            }
        }
        Ok(Self {})
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
