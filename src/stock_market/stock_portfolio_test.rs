#[cfg(test)]
use crate::stock_market::stock_portfolio::StockPortfolio;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[test]
fn it_gets_capital_gains_and_percent() {
    let capital_gains = StockPortfolio::get_capital_gains(dec!(1000), dec!(500));
    let capital_gains_percent = StockPortfolio::get_capital_gains_percent(dec!(1000), dec!(500));

    assert_eq!(capital_gains, dec!(500));
    assert_eq!(capital_gains_percent, dec!(100));
}

#[test]
fn it_gets_capital_losses_and_percent() {
    let capital_gains = StockPortfolio::get_capital_gains(dec!(500), dec!(1000));
    let capital_gains_percent = StockPortfolio::get_capital_gains_percent(dec!(500), dec!(1000));

    assert_eq!(capital_gains, dec!(-500));
    assert_eq!(capital_gains_percent, dec!(-50));
}
