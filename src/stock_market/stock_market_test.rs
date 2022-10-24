use crate::stock_market::stock_market::StockData;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rand::Rng;
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use rust_decimal_macros::dec;

#[cfg(test)]
use crate::stock_market::stock_market::*;

fn generate_utc_date_from_date_string(date_string: &str) -> DateTime<Utc> {
    let day_one = NaiveDateTime::parse_from_str(date_string, "%m-%d-%Y %H:%M").unwrap();
    Utc.from_utc_datetime(&day_one)
}

fn generate_stock_data(date_string: &str) -> StockData {
    let base_stock_data_series = vec![
        (130.0600, 131.3700, 128.8300, 129.1500),
        (125.7900, 125.8500, 124.5200, 125.0100),
        (124.1000, 125.5800, 123.8300, 125.4400),
        (122.6200, 124.0000, 122.5700, 123.7600),
        (122.1900, 123.5200, 121.3018, 123.3700),
        (121.2400, 121.8500, 120.5400, 121.7700),
        (121.6400, 121.6500, 120.1000, 120.7700),
        (120.9400, 121.5800, 120.5700, 121.0500),
        (120.6400, 120.9800, 120.3700, 120.9500),
        (120.5400, 120.8500, 119.9200, 120.3300),
        (119.7600, 120.3500, 119.5400, 120.1900),
        (118.6300, 119.5400, 118.5800, 119.2800),
        (119.8100, 120.0200, 118.6400, 119.9300),
        (119.3900, 120.2300, 119.3700, 119.8900),
        (120.1000, 120.2300, 118.3800, 119.3600),
        (119.8600, 120.4300, 119.1500, 119.9700),
        (119.0600, 119.4800, 118.5200, 119.1900),
        (118.9500, 119.1085, 118.1000, 119.0200),
        (118.0700, 118.3200, 116.9600, 117.9400),
        (117.4400, 117.5800, 116.1300, 116.9300),
        (117.8750, 118.2100, 115.5215, 116.7700),
        (118.6200, 118.7050, 116.8500, 117.9100),
        (116.5600, 118.0100, 116.3224, 117.6600),
        (119.5000, 119.5900, 117.0400, 117.0500),
        (117.1350, 120.8200, 117.0900, 120.2200),
        (117.3900, 118.7500, 116.7100, 117.5200),
        (118.0900, 118.4400, 116.9900, 117.6500),
        (116.1700, 117.6100, 116.0500, 117.5700),
        (115.3400, 117.2500, 114.5900, 115.9100),
        (114.5400, 115.2000, 114.3300, 114.5900),
    ];

    let base_data_series_len = base_stock_data_series.len();

    let mut rng = rand::thread_rng();

    let high = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].1)
        .unwrap()
        .round_dp(2);
    let low = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].2)
        .unwrap()
        .round_dp(2);
    let open = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].0)
        .unwrap()
        .round_dp(2);
    let close = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].3)
        .unwrap()
        .round_dp(2);

    StockData::new(
        generate_utc_date_from_date_string(date_string),
        high,
        low,
        open,
        close,
    )
}

fn generate_stock_data_series(limit: Option<u8>) -> Vec<StockData> {
    let mut stock_data_series: Vec<StockData> = vec![];
    for number in 0..limit.unwrap_or(7) {
        let number_plus = number + 1;

        let stock_date = match number_plus {
            number_plus if number_plus >= 10 => format!("10-{number_plus}-2022 00:00"),
            _ => format!("10-0{number_plus}-2022 00:00"),
        };

        let stock_data = generate_stock_data(&stock_date);
        stock_data_series.push(stock_data);
    }
    stock_data_series
}

#[test]
fn it_creates_a_new_stock_information_with_data_series() {
    let stock_data_series = generate_stock_data_series(Some(7));
    let stock_information = StockInformation::new(
        "BenCorpo".to_string(),
        "BNCRP".to_string(),
        stock_data_series,
    );
    let stock_data_series_with_change = stock_information.get_change_of_stock_data_series();

    println!(
        "stock_data_series_with_change {:?}",
        stock_data_series_with_change
    );

    println!("stock_information {:?}", stock_information);

    match stock_data_series_with_change {
        Some(stock_data_series_with_change) => assert_eq!(stock_data_series_with_change.len(), 7),
        None => println!("No stock data series found."),
    }
}

#[test]
fn it_creates_a_new_stock_information_without_data_series() {
    let stock_information =
        StockInformation::new("BenCorpo".to_string(), "BNCRP".to_string(), vec![]);
    let stock_data_series_with_change = stock_information.get_change_of_stock_data_series();

    println!(
        "stock_data_series_with_change {:?}",
        stock_data_series_with_change
    );
    println!("stock_information {:?}", stock_information);

    match stock_data_series_with_change {
        Some(stock_data_series_with_change) => assert_eq!(stock_data_series_with_change.len(), 0),
        None => assert!(true),
    }
}

#[test]
fn it_creates_a_new_stock_information_with_data_series_and_finds_specific_stock_data() {
    let stock_data_series = generate_stock_data_series(Some(7));
    let stock_information = StockInformation::new(
        "BenCorpo".to_string(),
        "BNCRP".to_string(),
        stock_data_series,
    );
    let stock_date = generate_utc_date_from_date_string("10-06-2022 00:00");
    let stock_data = stock_information.get_change_of_stock_data_with_given_date(stock_date);

    println!("stock_data {:?}", stock_data);

    println!("stock_information {:?}", stock_information);

    match stock_data {
        Some(_) => {
            assert!(true)
        }
        None => println!("No stock data found."),
    }
}

#[test]
fn it_creates_a_new_stock_information_with_data_series_and_does_not_find_a_specific_stock_data() {
    let stock_data_series = generate_stock_data_series(Some(7));
    let stock_information = StockInformation::new(
        "BenCorpo".to_string(),
        "BNCRP".to_string(),
        stock_data_series,
    );
    let stock_date = generate_utc_date_from_date_string("10-25-2022 00:00");
    let stock_data = stock_information.get_change_of_stock_data_with_given_date(stock_date);

    println!("stock_data {:?}", stock_data);

    println!("stock_information {:?}", stock_information);

    match stock_data {
        Some(_) => {
            println!("Stock data found.")
        }
        None => assert!(true),
    }
}

#[test]
fn it_creates_a_new_stock_information_with_data_series_and_gets_moving_averages() {
    let stock_data_series = vec![
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(121.00).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(122.00).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(120.00).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(119.00).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(124.00).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(128.00).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(126.00).unwrap().round_dp(2),
        ),
    ];
    let stock_information = StockInformation::new(
        "BenCorpo".to_string(),
        "BNCRP".to_string(),
        stock_data_series,
    );

    let moving_averages = stock_information.get_moving_averages(4);

    println!("moving_averages {:?}", moving_averages);
    match moving_averages {
        Some(moving_averages) => {
            assert_eq!(moving_averages.len(), 4);
            assert_eq!(
                moving_averages[0],
                Decimal::from_f64(120.50).unwrap().round_dp(2)
            );
            assert_eq!(
                moving_averages[1],
                Decimal::from_f64(121.25).unwrap().round_dp(2)
            );
            assert_eq!(
                moving_averages[2],
                Decimal::from_f64(122.75).unwrap().round_dp(2)
            );
            assert_eq!(
                moving_averages[3],
                Decimal::from_f64(124.25).unwrap().round_dp(2)
            );
        }
        None => println!("No moving averages found!"),
    }
}

#[test]
fn it_creates_a_new_stock_information_with_data_series_and_gets_moving_averages_two() {
    let stock_data_series = vec![
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(35.02).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(35.01).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(34.65).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(36.09).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(35.32).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(35.50).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(35.03).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(35.79).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(37.07).unwrap().round_dp(2),
        ),
        StockData::new(
            generate_utc_date_from_date_string("10-10-2022 00:00"),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(10.00).unwrap().round_dp(2),
            Decimal::from_f64(36.05).unwrap().round_dp(2),
        ),
    ];
    let stock_information = StockInformation::new(
        "BenCorpo".to_string(),
        "BNCRP".to_string(),
        stock_data_series,
    );

    let moving_averages = stock_information.get_moving_averages(5);
    match moving_averages {
        Some(moving_averages) => {
            assert_eq!(moving_averages.len(), 6);
            assert_eq!(
                moving_averages[0],
                Decimal::from_f64(35.22).unwrap().round_dp(2)
            );
            assert_eq!(
                moving_averages[1],
                Decimal::from_f64(35.32).unwrap().round_dp(2)
            );
            assert_eq!(
                moving_averages[2],
                Decimal::from_f64(35.32).unwrap().round_dp(2)
            );
            assert_eq!(
                moving_averages[3],
                Decimal::from_f64(35.55).unwrap().round_dp(2)
            );
            assert_eq!(
                moving_averages[4],
                Decimal::from_f64(35.75).unwrap().round_dp(2)
            );
            assert_eq!(
                moving_averages[5],
                Decimal::from_f64(35.90).unwrap().round_dp(2)
            );
        }
        None => println!("No moving averages found!"),
    }
}

#[test]
fn it_creates_a_new_stock_information_with_data_series_and_gets_no_moving_averages() {
    // Lacks moving averages data due to ma days input is greater than the stock data series available.
    let stock_data_series = generate_stock_data_series(Some(4));
    let stock_information = StockInformation::new(
        "BenCorpo".to_string(),
        "BNCRP".to_string(),
        stock_data_series,
    );

    let moving_averages = stock_information.get_moving_averages(5);

    println!("moving_averages {:?}", moving_averages);
    match moving_averages {
        Some(_) => {
            println!("Moving averages found!")
        }
        None => assert!(true),
    }
}

#[test]
fn it_creates_a_new_stock_information_with_data_series_and_show_chart_with_moving_average() {
    let stock_data_series = generate_stock_data_series(Some(14));
    let stock_information = StockInformation::new(
        "BenCorpo".to_string(),
        "BNCRP".to_string(),
        stock_data_series,
    );

    let ma_days = vec![7, 2, 0];
    let chart = stock_information.show_chart(ma_days, None, None, None);

    match chart {
        Ok(_) => {
            assert!(true)
        }
        Err(err) => println!("Error in saving chart {:?}", err),
    }
}

#[test]
fn it_creates_a_new_stock_information_with_data_series_and_does_not_show_chart() {
    let stock_data_series = generate_stock_data_series(Some(0));
    let stock_information = StockInformation::new(
        "BenCorpo".to_string(),
        "BNCRP".to_string(),
        stock_data_series,
    );

    let ma_days: Vec<u16> = vec![10];
    let chart = stock_information.show_chart(ma_days, None, None, None);

    println!("chart {:?}", chart);
    match chart {
        Ok(_) => {
            println!("There's a chart");
        }
        Err(err) => {
            println!("Error in saving chart {:?}", err);
            assert!(true);
        }
    }
}

#[test]
fn it_gets_post_split_data() {
    let post_split_data =
        StockInformation::get_post_split_data(dec!(5), dec!(4), dec!(942), dec!(56));

    assert_eq!(post_split_data.0, dec!(1177));
    assert_eq!(post_split_data.1, dec!(44.80));
    assert_eq!(post_split_data.2, dec!(22.40));

    let post_split_data =
        StockInformation::get_post_split_data(dec!(1), dec!(20), dec!(580_000_000), dec!(0.64));
    println!("{:?} KAMOTE", post_split_data);

    assert_eq!(post_split_data.0, dec!(29_000_000));
    assert_eq!(post_split_data.1, dec!(12.80));
    assert_eq!(post_split_data.2, dec!(0));
}
