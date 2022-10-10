#[cfg(test)]
use crate::stock_market::business_organization::*;

#[test]
fn it_creates_a_new_organization_with_owners() {
    let mark = Owner::new("Mark".to_string(), 250.0);
    let benedict = Owner::new("Benedict".to_string(), 200.0);
    let ben = Owner::new("Ben".to_string(), 150.25);

    let owners = vec![mark, benedict, ben];
    let organization = Corporation::new(owners, Some("KamoteCorp".to_string()), Some(100_000));

    assert_eq!(organization.owners.len(), 3);
}

#[test]
fn it_creates_a_new_organization_and_computes_total_investment() {
    let mark = Owner::new("Mark".to_string(), 250.0);
    let benedict = Owner::new("Benedict".to_string(), 200.0);
    let ben = Owner::new("Ben".to_string(), 150.25);

    let owners = vec![mark, benedict, ben];
    let organization = Corporation::new(owners, None, Some(100_000));

    assert_eq!(organization.total_investment, 250.0 + 200.0 + 150.25);
}

#[test]
fn it_creates_a_new_organization_and_gets_ownership_percentage_of_owner() {
    let mark = Owner::new("Mark".to_string(), 250.0);
    let benedict = Owner::new("Benedict".to_string(), 200.0);
    let ben = Owner::new("Ben".to_string(), 150.25);

    let owners = vec![mark, benedict, ben];
    let organization = Corporation::new(owners, None, Some(100_000));

    let ownership_percentage = organization.get_owner_ownership_percentage_by_investment("Mark");

    assert_eq!(ownership_percentage, 42.0);
}

#[test]
fn it_creates_a_new_organization_and_tries_to_get_ownership_percentage_of_unregistered_owner() {
    let mark = Owner::new("Mark".to_string(), 250.0);
    let benedict = Owner::new("Benedict".to_string(), 200.0);
    let ben = Owner::new("Ben".to_string(), 150.25);

    let owners = vec![mark, benedict, ben];
    let organization = Corporation::new(owners, None, Some(100_000));

    let ownership_percentage = organization.get_owner_ownership_percentage_by_investment("MEMA");

    assert_eq!(ownership_percentage, 0.0);
}

#[test]
fn it_creates_a_new_organization_and_gets_owner_shares_by_ownership_percentage() {
    let mark = Owner::new("Mark".to_string(), 250.0);
    let benedict = Owner::new("Benedict".to_string(), 200.0);
    let ben = Owner::new("Ben".to_string(), 150.25);

    let owners = vec![mark, benedict, ben];
    let organization = Corporation::new(owners, None, Some(100_000));

    let owner_shares = organization.get_owner_shares_by_ownership_percentage("Mark");

    assert_eq!(owner_shares, 42000);
}

#[test]
fn it_creates_a_new_organization_and_gets_owner_ownership_percentages() {
    let mark = Owner::new("Mark".to_string(), 250.0);
    let benedict = Owner::new("Benedict".to_string(), 200.0);
    let ben = Owner::new("Ben".to_string(), 150.25);

    let owners = vec![mark, benedict, ben];
    let organization = Corporation::new(owners, None, Some(100_000));

    let owner_ownership_percentages = organization.get_owners_ownership_percentages();

    let mark = owner_ownership_percentages[0]
        .get("percentage")
        .unwrap()
        .f32();
    let benedict = owner_ownership_percentages[1]
        .get("percentage")
        .unwrap()
        .f32();
    let ben = owner_ownership_percentages[2]
        .get("percentage")
        .unwrap()
        .f32();

    assert_eq!(owner_ownership_percentages.len(), 3);
    assert_eq!(mark, &42.0);
    assert_eq!(benedict, &33.0);
    assert_eq!(ben, &25.0);
}

#[test]
fn it_creates_a_new_organization_and_detects_major_and_not_major_shareholder() {
    let mark = Owner::new("Mark".to_string(), 250.0);
    let benedict = Owner::new("Benedict".to_string(), 200.0);
    let ben = Owner::new("Ben".to_string(), 150.25);
    let hello = Owner::new("Hello".to_string(), 500.25);

    let owners = vec![mark, benedict, ben, hello];

    let organization = Corporation::new(owners, None, Some(100_000));

    let is_major_shareholder = organization.is_owner_majority_shareholder("Mark");
    println!("Mark {}", is_major_shareholder);
    assert_eq!(is_major_shareholder, false);

    let is_major_shareholder = organization.is_owner_majority_shareholder("Ben");
    println!("Ben {}", is_major_shareholder);
    assert_eq!(is_major_shareholder, false);

    let is_major_shareholder = organization.is_owner_majority_shareholder("Benedict");
    println!("Benedict {}", is_major_shareholder);
    assert_eq!(is_major_shareholder, false);

    let is_major_shareholder = organization.is_owner_majority_shareholder("Wala");
    println!("Wala {}", is_major_shareholder);
    assert_eq!(is_major_shareholder, false);

    let is_major_shareholder = organization.is_owner_majority_shareholder("Hello");
    println!("Hello {}", is_major_shareholder);
    assert_eq!(is_major_shareholder, true);

    println!("Organization {:?}", organization);
}
