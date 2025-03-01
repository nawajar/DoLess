# DoLess - Procedural Macro for Struct Mapping 🦀

`DoLess` is a Rust **procedural macro** that allows structs to be initialized from a `HashMap<String, String>`. It automatically maps field values, providing **type-safe conversions**.

## 🚀 Features
- 🏢 **Auto-implements `From<HashMap<String, String>>`** for structs.
- 🔄 **Supports common Rust types** (`String`, `u8`, `u16`, `i32`, `f64`, `Option`, `Vec<T>`, `Vec<Option<T>>`, etc.).
- ❌ **Compile-time errors for unsupported types**.
- ✅ **Default values for missing fields**.
- ⚙ **Supports nested struct parsing** with `.` notation.

---

## 🛆 Installation
Add `DoLess` to your `Cargo.toml`:

```toml
[dependencies]
doless = "0.3.0"
```

## 👺 Usage

### Basic Struct Mapping
```rust
use doless::FromHashMap;
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

### Nested Struct Support
```rust
use doless::FromHashMap;
use std::collections::HashMap;

#[derive(FromHashMap, Debug)]
struct Car {
    model: String,
    brand: String,
    number: u8,
    details: CarDetails,  // ✅ Nested Struct Support
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

    // ✅ Nested Fields with Prefix Notation
    data.insert("details.name".to_string(), "Skyline".to_string());
    data.insert("details.description".to_string(), "Legendary Sports Car".to_string());

    let car: Car = Car::from(data);
    println!("{:?}", car);
}
```

### Support for `Vec<T>` and `Vec<Option<T>>`
```rust
use doless::FromHashMap;
use std::collections::HashMap;

#[derive(FromHashMap, Debug)]
struct ItemCollection {
    items: Vec<String>,         // ✅ Supports Vec<String>
    numbers: Vec<u8>,           // ✅ Supports Vec<u8>
    optional_items: Vec<Option<String>>,  // ✅ Supports Vec<Option<T>>
}

fn main() {
    let mut data = HashMap::new();
    data.insert("items".to_string(), "apple, banana, orange".to_string());
    data.insert("numbers".to_string(), "1,2,3".to_string());
    data.insert("optional_items".to_string(), "apple,,orange".to_string()); // Empty string = None

    let collection: ItemCollection = ItemCollection::from(data);
    println!("{:?}", collection);
}
```

### Expected Output
```rust
ItemCollection {
    items: ["apple", "banana", "orange"],
    numbers: [1, 2, 3],
    optional_items: [Some("apple"), None, Some("orange")],
}
```

---

## 🚀 Why Use DoLess?
- **Simple & Lightweight** — No runtime dependencies, just pure Rust.
- **Declarative API** — Uses procedural macros to generate efficient `From<HashMap<String, String>>` implementations.
- **Type-Safe & Extensible** — Ensures correct conversions and supports nesting.

### ⚙ Roadmap
- [x] Basic primitive types mapping
- [x] Nested struct support
- [x] `Vec<T>` and `Vec<Option<T>>` support
- [ ] Custom conversion support
- [ ] Error handling improvements

---

**Happy coding! ✨**

