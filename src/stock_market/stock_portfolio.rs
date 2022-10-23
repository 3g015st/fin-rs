use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Debug)]
pub struct StockPortfolio {}

impl StockPortfolio {
    pub fn get_capital_gains(selling_price: Decimal, purchase_price: Decimal) -> Decimal {
        selling_price - purchase_price
    }
    pub fn get_capital_gains_percent(selling_price: Decimal, purchase_price: Decimal) -> Decimal {
        let capital_gains = StockPortfolio::get_capital_gains(selling_price, purchase_price);
        (capital_gains / purchase_price) * dec!(100)
    }
}
