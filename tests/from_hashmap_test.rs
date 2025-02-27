use std::collections::HashMap;
use structify::FromHashMap;

#[derive(FromHashMap, Debug)]
struct Car {
    model: String,
    brand: String,
    an_option_field: Option<String>,
    number: u8,
    details: CarDetails,
}

#[derive(FromHashMap, Debug)]
struct CarDetails {
    name: String,
    description: String,
}

#[test]
fn test_from_hashmap() {
    let mut data = HashMap::new();
    data.insert("model".to_string(), "GT-R".to_string());
    data.insert("brand".to_string(), "Nissan".to_string());
    data.insert("number".to_string(), "8".to_string());
    data.insert("details.name".to_string(), "v8engine".to_string());
    data.insert("details.description".to_string(), "500hp".to_string());

    let car: Car = Car::from(data);

    assert_eq!(car.model, "GT-R");
    assert_eq!(car.brand, "Nissan");
    assert_eq!(car.number, 0b1000);
    assert_eq!(car.an_option_field, None);
    assert_eq!(car.details.name, "v8engine");
    assert_eq!(car.details.description, "500hp");
}

