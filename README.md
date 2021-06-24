# Ascii Converter

---

## Description
This project is a library for converting between different Ascii representations in the Rust language. This is made for rust programs that need to convert an ascii value into another representation. 

Currently supported representations
- Binary
- Decimal
- Characters

Full Documentation for this library can be found [here]()

---

## Installation

Add to this to your projects Cargo.toml:

```toml
[dependencies]
ascii_converter = "0.1.0"
```

---

## Usage
This library consists of several functions that follow the same simplistic convention, input the data and the new representation is returned.

below is a program that converts text to binary and decimal. this code can be found in the examples/conversion.rs 

```rust
use ascii_converter::*;

fn main() {

    let mut name = String::new();
    println!("Enter name: ");

    //reads user input and assigns it to the name variable
    std::io::stdin().read_line(&mut name).unwrap();

    //remove the /n from the end of name
    name.truncate(name.len() -1);

    //outputs the binary representation
    println!("* {} in Binary: {:?}", name, string_to_binary(&name).unwrap());
    
    //outputs the decimal representation
    println!("* {} in Decimal: {:?}", name, string_to_decimals(&name).unwrap());

}
```
---

## License

[MIT](LICENSE)

