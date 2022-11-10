#[cfg(test)]
use crate::business_modelling::linreg;

#[test]
fn it_successfully_computes_linear_regression() {
    let domain = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let range = vec![9.0, 14.0, 7.0, 18.0, 27.0];

    let result = linreg::Linreg::linear_regress(&domain, &range);
    match result {
        Ok((mx, b)) => {
            assert_eq!(mx, 2.0);
            assert_eq!(b, 3.0);
        }
        Err(err) => println!("Error in linear regress {:?}", err),
    }
}

#[test]
fn it_returns_error_because_domain_range_not_same_len() {
    let domain = vec![2.0, 4.0, 6.0, 8.0];
    let range = vec![9.0, 14.0, 7.0, 18.0, 27.0];

    let result = linreg::Linreg::linear_regress(&domain, &range);
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

    let result = linreg::Linreg::linear_regress(&domain, &range);
    match result {
        Ok(_) => {
            println!("Success");
        }
        Err(err) => assert_eq!(err.to_string(), "Insufficient series lengths"),
    }

    let domain = vec![9.0, 14.0, 7.0, 18.0, 27.0];
    let range = vec![];

    let result = linreg::Linreg::linear_regress(&domain, &range);
    match result {
        Ok(_) => {
            println!("Success");
        }
        Err(err) => assert_eq!(err.to_string(), "Insufficient series lengths"),
    }
}
