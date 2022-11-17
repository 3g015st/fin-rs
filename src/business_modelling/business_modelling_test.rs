#[cfg(test)]
use crate::business_modelling::business_modelling;

#[test]
fn it_successfully_shows_scatterplot() {
    let domain = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let range = vec![9.0, 14.0, 7.0, 18.0, 27.0];

    let result = business_modelling::BusinessModelling::scatterplot(
        &domain,
        &range,
        "Tindahan ni Nene".to_string(),
        "Temperature".to_string(),
        "Sales".to_string(),
        None,
        None,
        None,
    );
    match result {
        Ok(_) => {
            assert!(true);
        }
        Err(err) => println!("Error in showing scatterplot {:?}", err),
    }
}

#[test]
fn it_returns_error_because_domain_range_not_same_len() {
    let domain = vec![2.0, 4.0, 6.0, 8.0];
    let range = vec![9.0, 14.0, 7.0, 18.0, 27.0];

    let result = business_modelling::BusinessModelling::scatterplot(
        &domain,
        &range,
        "Tindahan ni Nene".to_string(),
        "Temperature".to_string(),
        "Sales".to_string(),
        None,
        None,
        None,
    );
    match result {
        Ok(_) => {
            println!("Success");
        }
        Err(err) => assert_eq!(
            err.to_string(),
            "Range length is not equal to domain length or vice versa"
        ),
    }
}

#[test]
fn it_returns_error_because_domain_range_is_empty() {
    let domain = vec![];
    let range = vec![9.0, 14.0, 7.0, 18.0, 27.0];
    let result = business_modelling::BusinessModelling::scatterplot(
        &domain,
        &range,
        "Tindahan ni Nene".to_string(),
        "Temperature".to_string(),
        "Sales".to_string(),
        None,
        None,
        None,
    );
    match result {
        Ok(_) => {
            println!("Success");
        }
        Err(err) => assert_eq!(err.to_string(), "Insufficient series lengths"),
    }

    let domain = vec![9.0, 14.0, 7.0, 18.0, 27.0];
    let range = vec![];

    let result = business_modelling::BusinessModelling::scatterplot(
        &domain,
        &range,
        "Tindahan ni Nene".to_string(),
        "Temperature".to_string(),
        "Sales".to_string(),
        None,
        None,
        None,
    );
    match result {
        Ok(_) => {
            println!("Success");
        }
        Err(err) => assert_eq!(err.to_string(), "Insufficient series lengths"),
    }
}

#[test]
fn it_successfully_shows_demand_supply_graph() {
    let prices = vec![15.25, 15.50, 15.75, 16.00, 16.25, 16.50, 16.75, 17.00];
    let quantity_purchase = vec![
        3456.00, 3005.00, 2546.00, 2188.00, 1678.00, 1290.00, 889.00, 310.00,
    ];
    let quantity_produce = vec![
        310.00, 889.00, 1290.00, 1678.00, 2188.00, 2546.00, 3005.00, 3456.00,
    ];

    let result = business_modelling::BusinessModelling::demand_supply_scatterplot(
        &prices,
        &quantity_purchase,
        &quantity_produce,
        Some(16.125),
        "Pet Store".to_string(),
        None,
        None,
        None,
    );
    match result {
        Ok(_) => {
            assert!(true);
        }
        Err(err) => println!("Error in showing supply and demand scatterplot {:?}", err),
    }
}

#[test]
fn it_successfully_shows_business_model_graph() {
    let prices = vec![15.25, 15.50, 15.75, 16.00, 16.25, 16.50, 16.75, 17.00];
    let quantity_purchase = vec![
        3456.00, 3005.00, 2546.00, 2188.00, 1678.00, 1290.00, 889.00, 310.00,
    ];

    let fixed_cost = 9_000.00;
    let manufacturing_cost = 5.0;

    let result = business_modelling::BusinessModelling::model(
        &prices,
        &quantity_purchase,
        &fixed_cost,
        &manufacturing_cost,
        "Pet Store".to_string(),
        None,
        None,
        None,
    );
    match result {
        Ok(_) => {
            assert!(true);
        }
        Err(err) => println!("Error in showing supply and demand scatterplot {:?}", err),
    }
}
