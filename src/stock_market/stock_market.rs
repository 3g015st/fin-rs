use std::{error::Error, fs};

use chrono::{prelude::*, Duration};
use plotters::{
    prelude::{
        BitMapBackend, CandleStick, ChartBuilder, IntoDrawingArea, PathElement, SeriesLabelPosition,
    },
    series::LineSeries,
    style::{
        full_palette::{ORANGE, PURPLE},
        Color, IntoFont, RGBColor, BLUE, GREEN, RED, WHITE,
    },
};
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
    pub fn show_chart(
        &self,
        ma_days: Vec<u16>,
        directory: Option<String>,
        height: Option<u32>,
        width: Option<u32>,
    ) -> Result<bool, Box<dyn Error>> {
        let stock_data_series = &self.stock_data_series;
        if stock_data_series.len() == 0 {
            Err("Insufficient stock data series length")?;
        }

        if ma_days.len() > 3 {
            Err("Exceeded the limit of moving averages to plot")?;
        }

        let dt = Utc::now();
        let timestamp: i64 = dt.timestamp();

        let dir = directory.unwrap_or("chart_outputs".to_string());

        fs::create_dir_all(&dir)?;

        let filepath = format!("{}/{}_candlestick_chart.png", &dir, timestamp);
        let drawing_area =
            BitMapBackend::new(&filepath, (height.unwrap_or(1024), width.unwrap_or(768)))
                .into_drawing_area();

        drawing_area.fill(&WHITE)?;

        let candlesticks = stock_data_series.iter().map(|stock_data| {
            CandleStick::new(
                stock_data.date.date(),
                stock_data.open.to_f64().unwrap(),
                stock_data.high.to_f64().unwrap(),
                stock_data.low.to_f64().unwrap(),
                stock_data.close.to_f64().unwrap(),
                GREEN.filled(),
                RED.filled(),
                25,
            )
        });

        let stock_data_series_last_day_idx = stock_data_series.len() - 1;

        let (from_date, to_date) = (
            stock_data_series[0].date.date() - Duration::days(1),
            stock_data_series[stock_data_series_last_day_idx]
                .date
                .date()
                + Duration::days(1),
        );

        let mut chart_builder = ChartBuilder::on(&drawing_area);

        let min_low_price = stock_data_series
            .iter()
            .map(|stock_data| stock_data.low)
            .min()
            .unwrap();
        let max_high_price = stock_data_series
            .iter()
            .map(|stock_data| stock_data.high)
            .max()
            .unwrap();

        let x_spec = from_date..to_date;
        let y_spec = min_low_price.to_f64().unwrap()..max_high_price.to_f64().unwrap();
        let caption = format!("{} Stock Price Movement", &self.company_name);
        let font_style = ("sans-serif", 25.0).into_font();

        let mut chart = chart_builder
            .x_label_area_size(40)
            .y_label_area_size(40)
            .caption(caption, font_style.clone())
            .build_cartesian_2d(x_spec, y_spec)?;

        chart.configure_mesh().light_line_style(&WHITE).draw()?;

        chart.draw_series(candlesticks)?;

        // Draw moving averages lines
        if ma_days.len() > 0 {
            // Parallel computed moving averages
            let moving_averages_2d: Vec<_> = ma_days
                .into_iter()
                .filter(|ma_day| ma_day > &&0)
                .map(|ma_day| {
                    let moving_averages = self.get_moving_averages(ma_day.clone());

                    match moving_averages {
                        Some(moving_averages) => return (ma_day, moving_averages),
                        None => return (ma_day, Vec::with_capacity(0)),
                    }
                })
                .collect();

            for (idx, ma_tuple) in moving_averages_2d.iter().enumerate() {
                let (ma_day, moving_averages) = ma_tuple;
                let mut ma_line_data: Vec<(Date<Utc>, f64)> = Vec::with_capacity(3);
                let ma_len = moving_averages.len();

                for i in 0..ma_len {
                    // Let start moving average day at the day where adequate data has been formed.
                    let ma_day = i + ma_day.to_usize().unwrap() - 1;
                    ma_line_data.push((
                        stock_data_series[ma_day].date.date(),
                        moving_averages[i].to_f64().unwrap(),
                    ));
                }

                if ma_len > 0 {
                    let chosen_color = [BLUE, PURPLE, ORANGE][idx];

                    let line_series_label = format!("SMA {}", &ma_day);

                    let legend = |color: RGBColor| {
                        move |(x, y)| PathElement::new([(x, y), (x + 20, y)], color)
                    };

                    let sma_line = LineSeries::new(ma_line_data, chosen_color.stroke_width(2));

                    // Fill in moving averages line data series
                    chart
                        .draw_series(sma_line)
                        .unwrap()
                        .label(line_series_label)
                        .legend(legend(chosen_color));
                }

                // Display SMA Legend
                chart
                    .configure_series_labels()
                    .position(SeriesLabelPosition::UpperLeft)
                    .label_font(font_style.clone())
                    .draw()
                    .unwrap();
            }
        }

        drawing_area.present().expect(&format!(
            "Cannot write into {:?}. Directory does not exists.",
            &dir
        ));

        println!("Result has been saved to {}", filepath);

        Ok(true)
    }
}
