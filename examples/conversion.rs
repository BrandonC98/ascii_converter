use ascii_converter::*;
use std::io::*;

fn main() {

    let mut name = String::new();
    print!("Enter name: ");
    stdout().flush();
    //reads user input and assigns it to the name variable
    stdin().read_line(&mut name).unwrap();

    let name = name.trim();

    //outputs the binary representation
    println!("* {} in Binary: {:?}", name, string_to_binary(&name).unwrap());
    
    //outputs the decimal representation
    println!("* {} in Decimal: {:?}", name, string_to_decimals(&name).unwrap());

}