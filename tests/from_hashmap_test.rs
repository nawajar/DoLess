use doless::FromHashMap;
use std::collections::HashMap;

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

#[derive(FromHashMap, Debug)]
struct VecStruct {
    vec_string: Vec<String>,
    vec_u8: Vec<u8>,
    vec_option: Vec<Option<String>>,
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

#[test]
fn test_from_vec() {
    let mut data = HashMap::new();
    data.insert("vec_string".to_string(), "hello, world, rust".to_string());
    data.insert("vec_u8".to_string(), "1, 2, 999".to_string());
    data.insert("vec_option".to_string(), "1,2,,".to_string());
    let car_details: VecStruct = VecStruct::from(data);

    println!("{:?}", car_details);
    assert_eq!(car_details.vec_string.len(), 3);
    assert_eq!(car_details.vec_string, vec!["hello", "world", "rust"]);
    assert_eq!(car_details.vec_u8, vec![1, 2]); //u8 overflow
    assert_eq!(car_details.vec_option, vec![Some(String::from("1")), Some(String::from("2")), None, None]);
}
