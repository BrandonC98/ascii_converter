use ascii_converter::*;

fn main() {

    let mut input = String::new();
    println!("Enter text: ");
    std::io::stdin().read_line(&mut input).unwrap();
    input.truncate(input.len() -1);

    println!("* {} in Binary: {:?}", input, string_to_binary(&input).unwrap());
    println!("* {} in Decimal: {:?}", input, string_to_decimals(&input).unwrap());

}