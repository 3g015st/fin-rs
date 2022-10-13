use chrono::prelude::*;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
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
    pub fn get_change_of_stock_data_with_given_date(
        &self,
        search_date: DateTime<Utc>,
    ) -> Option<StockData> {
        let stock_data_series_with_change = self.get_change_of_stock_data_series();

        match stock_data_series_with_change {
            Some(stock_data_series_with_change) => {
                return stock_data_series_with_change
                    .into_iter()
                    .find(|stock_data| stock_data.date == search_date);
            }
            None => None,
        }
    }

    pub fn get_moving_averages(&self, ma_days: u16) -> Option<Vec<Decimal>> {
        if self.stock_data_series.len() == 0 {
            return None;
        }

        let mut moving_averages: Vec<Decimal> = vec![];
        let closing_prices = self
            .stock_data_series
            .iter()
            .map(|stock_data| stock_data.close)
            .collect::<Vec<Decimal>>();

        // No moving averages to be computed since current closing price series is not sufficient to build based upon ma_days parameters.
        if closing_prices.len() < ma_days.into() {
            return None;
        }

        let ma_days_idx_end = ma_days - 1;

        let ma_days_decimal = Decimal::from_u16(ma_days).unwrap();
        let mut sum = dec!(0.0);
        for x in 0..=ma_days_idx_end {
            let closing_price = &closing_prices[x.to_usize().unwrap()];
            sum = sum + closing_price;
        }

        let first_moving_average_day = sum / ma_days_decimal;
        moving_averages.push(first_moving_average_day.round_dp(2));

        if closing_prices.len() == ma_days.into() {
            return Some(moving_averages);
        }

        let mut idx: usize = 0;
        let mut tail_closing_day_idx: usize = (ma_days_idx_end + 1).to_usize().unwrap();

        while tail_closing_day_idx != closing_prices.len() {
            let previous_moving_average = &moving_averages[idx];
            let head_closing_day_price = &closing_prices[idx] / ma_days_decimal;
            let tail_closing_day_price = &closing_prices[tail_closing_day_idx] / ma_days_decimal;
            let current_moving_average =
                previous_moving_average - head_closing_day_price + tail_closing_day_price;
            moving_averages.push(current_moving_average.round_dp(2));

            idx += 1;
            tail_closing_day_idx += 1;
        }

        return Some(moving_averages);
    }
}
