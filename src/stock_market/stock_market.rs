use chrono::prelude::*;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

// net_change and net_change_percent will be computed.
#[derive(Debug)]
pub struct StockData {
    date: DateTime<Utc>,
    high: Decimal,
    low: Decimal,
    open: Decimal,
    close: Decimal,
    net_change: Option<Decimal>,
    net_change_percent: Option<Decimal>,
}

impl StockData {
    pub fn new(
        date: DateTime<Utc>,
        high: Decimal,
        low: Decimal,
        open: Decimal,
        close: Decimal,
    ) -> Self {
        Self {
            date,
            high,
            low,
            open,
            close,
            net_change: None,
            net_change_percent: None,
        }
    }
}

#[derive(Debug)]
pub struct StockInformation {
    company_name: String,
    symbol: String,
    stock_data_series: Vec<StockData>,
}

impl StockInformation {
    pub fn new(company_name: String, symbol: String, stock_data_series: Vec<StockData>) -> Self {
        Self {
            company_name,
            symbol,
            stock_data_series,
        }
    }

    pub fn get_change_of_stock_data_series(&self) -> Option<Vec<StockData>> {
        if self.stock_data_series.len() == 0 {
            return None;
        }

        let mut stock_data_series_with_change: Vec<StockData> = vec![];
        for (index, stock_data) in self.stock_data_series.iter().enumerate() {
            if index == 0 {
                stock_data_series_with_change.push(StockData {
                    date: stock_data.date,
                    high: stock_data.high,
                    low: stock_data.low,
                    open: stock_data.open,
                    close: stock_data.close,
                    net_change: Some(dec!(0.0)),
                    net_change_percent: Some(dec!(0.0)),
                });
            } else {
                let previous_day_close = &self.stock_data_series[index - 1].close;
                let current_day_close = stock_data.close;

                let net_change = current_day_close - previous_day_close;
                let net_change_percent = (net_change / previous_day_close) * dec!(100.0);

                stock_data_series_with_change.push(StockData {
                    date: stock_data.date,
                    high: stock_data.high,
                    low: stock_data.low,
                    open: stock_data.open,
                    close: stock_data.close,
                    net_change: Some(net_change.round_dp(2)),
                    net_change_percent: Some(net_change_percent.round_dp(2)),
                });
            }
        }

        Some(stock_data_series_with_change)
    }

    /*** TODO: Refactor. Since get_change_of_stock_data_series is O(n), by calling that function inside this function we're looping
     * 2x, first loop for building stock change data and second loop is for finding the stock data with specific date. 
     * What happens is Map -> Filter pattern.
    ***/
    pub fn get_change_of_stock_data_with_given_date(&self, search_date : DateTime<Utc>) -> Option<StockData> {
        let stock_data_series_with_change = self.get_change_of_stock_data_series();

        match stock_data_series_with_change {
            Some(stock_data_series_with_change) => {
                return stock_data_series_with_change.into_iter().find(|stock_data| stock_data.date == search_date);
            }
            None => None
        }
      
    }
}
