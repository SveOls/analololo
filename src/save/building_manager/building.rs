
use super::*;


#[derive(Debug)]
pub struct Building {
}

impl Building {
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {


        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "building" => {},
                "level" => {},
                "last_updated_level" => {},
                "state" => {},
                "salary_rate" => {},
                "production_methods" => {},
                "input_goods" => {},
                "output_goods" => {},
                "dead" => {},
                "active" => {},
                "establishment_date" => {},
                "staffing" => {},
                "previous_staffing" => {},
                "salaries" => {},
                "goods_cost" => {},
                "profit_after_reserves" => {},
                "profit_after_investments" => {},
                "income_taxes" => {},
                "dividends" => {},
                "input_goods_shortage" => {},
                "employee_transfers" => {},
                "throughput" => {},
                "timed_modifiers" => {},
                "auto_expands" => {},
                "dividends_taxes" => {},
                "failed_hires" => {},
                "balance_subsidies" => {},
                "government_dividends" => {},
                "construction_province" => {},
                "goods_sales" => {},
                "last_layoff_date" => {},
                "wage_subsidies" => {},
                "cash_reserves" => {},
                "trade_route_income" => {},
                "tariffs" => {},
                "slave_goods" => {},
                "slave_upkeep" => {},
                "subsidized" => {},
                "slave_basket" => {},
                a => println!("\t\t\t\t\"{a}\" => {{}},")
            }
        }
        Ok(Self {
        })
    }
    pub fn new_group(inp: ObjectReader<Utf8Encoding>) -> Result<HashMap<usize, Option<Self>>, Box<dyn Error>> {

        let mut ret = HashMap::new();

        for (key, _, value) in inp.fields() {
            ret.insert(key.read_str().parse()?, value.read_object().ok().map(|x| Building::new(x)).transpose()?);
        }
        Ok(ret)
    }
}
