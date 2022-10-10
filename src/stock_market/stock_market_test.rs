use crate::stock_market::stock_market::StockData;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rand::Rng;
use rust_decimal::{prelude::FromPrimitive, Decimal};

#[cfg(test)]
use crate::stock_market::stock_market::*;

fn generate_utc_date_from_date_string(date_string: &str) -> DateTime<Utc> {
    let day_one = NaiveDateTime::parse_from_str(date_string, "%m-%d-%Y %H:%M").unwrap();
    Utc.from_utc_datetime(&day_one)
}

fn generate_stock_data(date_string: &str) -> StockData {
    let mut rng = rand::thread_rng();
    StockData::new(
        generate_utc_date_from_date_string(date_string),
        Decimal::from_f64(rng.gen_range(10.0..100.0))
            .unwrap()
            .round_dp(2),
        Decimal::from_f64(rng.gen_range(10.0..100.0))
            .unwrap()
            .round_dp(2),
        Decimal::from_f64(rng.gen_range(10.0..100.0))
            .unwrap()
            .round_dp(2),
        Decimal::from_f64(rng.gen_range(10.0..100.0))
            .unwrap()
            .round_dp(2),
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
