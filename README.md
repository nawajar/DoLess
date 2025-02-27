# DoLess - Procedural Macro for Struct Mapping 🦀

`DoLess` is a Rust **procedural macro** that allows structs to be initialized from a `HashMap<String, String>`. It automatically maps field values, providing **type-safe conversions**.

## 🚀 Features
- 🏢 **Auto-implements `From<HashMap<String, String>>`** for structs.
- 🔄 **Supports common Rust types** (`String`, `u8`, `u16`, `i32`, `f64`, `Option`, etc.).
- ❌ **Compile-time errors for unsupported types**.
- ✅ **Default values for missing fields**.
- ⚙ **Supports nested struct parsing** with `.` notation.

---

## 📦 Installation
Add `DoLess` to your `Cargo.toml`:

```toml
[dependencies]
doless = "0.1.0"
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

### Expected Output
```rust
Car {
    model: "GT-R",
    brand: "Nissan",
    number: 8,
    details: CarDetails {
        name: "Skyline",
        description: "Legendary Sports Car"
    }
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
- [ ] Custom conversion support
- [ ] Error handling improvements

---

**Happy coding! ✨**

