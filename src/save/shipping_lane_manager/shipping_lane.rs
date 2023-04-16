
use super::*;


#[derive(Debug)]
pub struct ShippingLane {
}

impl ShippingLane {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {


        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "convoy_cost" => {},
                "object" => {},
                "country" => {},
                "shipping_lane_type" => {},
                "sea_nodes" => {},
                "begin_point" => {},
                "end_point" => {},
                a => println!("\t\t\t\t\"{a}\" => {{}},")
            }
        }
        Ok(Self {
        })
    }
    pub fn new_group(inp: ObjectReader<Utf8Encoding>) -> Result<HashMap<usize, Option<Self>>, Box<dyn Error>> {

        let mut ret = HashMap::new();

        for (key, _, value) in inp.fields() {
            ret.insert(key.read_str().parse()?, value.read_object().ok().map(|x| ShippingLane::new(x)).transpose()?);
        }
        Ok(ret)
    }
}
