use super::*;

#[derive(Debug, Default)]
pub struct Building {
    goods_cost: f64,
    goods_sales: f64,
    location: usize,
}

impl Building {
    pub fn goods_sales(&self) -> f64 {
        self.goods_sales
    }
    pub fn goods_cost(&self) -> f64 {
        self.goods_cost
    }
    pub fn goods_net(&self) -> f64 {
        self.goods_sales() - self.goods_cost()
    }
    pub fn location(&self) -> usize {
        self.location
    }
    pub fn new(inp: ObjectReader<Utf8Encoding>) -> Result<Self, Box<dyn Error>> {
        
        let mut ret = Building::default();
        
        for (key, _, value) in inp.fields() {
            match key.read_str().as_ref() {
                "building" => {}
                "level" => {}
                "last_updated_level" => {}
                "state" => ret.location = value.read_scalar()?.to_u64()? as usize,
                "salary_rate" => {}
                "production_methods" => {}
                "input_goods" => {}
                "output_goods" => {}
                "dead" => {}
                "active" => {}
                "establishment_date" => {}
                "staffing" => {}
                "previous_staffing" => {}
                "salaries" => {}
                "goods_cost" => ret.goods_cost = value.read_scalar()?.to_f64()?,
                "profit_after_reserves" => {}
                "profit_after_investments" => {}
                "income_taxes" => {}
                "dividends" => {}
                "input_goods_shortage" => {}
                "employee_transfers" => {}
                "throughput" => {}
                "timed_modifiers" => {}
                "auto_expands" => {}
                "dividends_taxes" => {}
                "failed_hires" => {}
                "balance_subsidies" => {}
                "government_dividends" => {}
                "construction_province" => {}
                "goods_sales" => ret.goods_sales = value.read_scalar()?.to_f64()?,
                "last_layoff_date" => {}
                "wage_subsidies" => {}
                "cash_reserves" => {}
                "trade_route_income" => {}
                "tariffs" => {}
                "slave_goods" => {}
                "slave_upkeep" => {}
                "subsidized" => {}
                "slave_basket" => {}
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
                    .map(|x| Building::new(x))
                    .transpose()?,
            );
        }
        Ok(ret)
    }
}
