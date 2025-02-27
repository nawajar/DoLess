
# DoLess - Procedural Macro for Struct Mapping 🦀

`Structify` is a Rust **procedural macro** that allows structs to be initialized from a `HashMap<String, String>`. It automatically maps field values, providing **type-safe conversions**.

## 🚀 Features
🏢 Auto-implements From<HashMap<String, String>> for structs.\
🔄 Supports common Rust types (String, u8, u16, i32, f64, Option, etc.).\
❌ Compile-time errors for unsupported types.\
✅ Default values for missing fields.\
⚙ Supports nested struct parsing with . notation.

---

## 📦 Installation


📌 Usage

```rust 
use structify::FromHashMap;
use std::collections::HashMap;

#[derive(FromHashMap, Debug, PartialEq)]
struct Car {
    model: String,
    year: u16,
}

fn main() {
    let mut data = HashMap::new();
    data.insert("model".to_string(), "GT-R".to_string());
    data.insert("year".to_string(), "2023".to_string());

    let car: Car = Car::from(data);
    println!("Car: Model = {}, Year = {}", car.model, car.year);
}

```


```rust
use structify::FromHashMap;
use std::collections::HashMap;

#[derive(FromHashMap, Debug)]
struct Car {
    model: String,
    brand: String,
    number: u8,
    details: CarDetails,  
}

#[derive(FromHashMap, Debug)]
struct CarDetails {
    name: String,
    description: String,
}

fn main() {
    let mut data = HashMap::new();
    data.insert("model".to_string(), "GT-R".to_string());
    data.insert("brand".to_string(), "Nissan".to_string());
    data.insert("number".to_string(), "8".to_string());

    data.insert("details.name".to_string(), "Skyline".to_string());
    data.insert("details.description".to_string(), "Legendary Sports Car".to_string());

    let car: Car = Car::from(data);
    println!("{:?}", car);
}
```
